use clap::{Arg, Command};
use osr_client::auth_server::AuthServerConfig;
use osr_client::shared::{AuthRequest, AuthResponse};
use tokio::net::TcpListener;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

fn load_tls_config(cert_path: &str, key_path: &str) -> Result<TlsAcceptor, Box<dyn std::error::Error>> {
    // Load certificate
    let cert_file = File::open(cert_path)?;
    let mut cert_reader = BufReader::new(cert_file);
    let certs = rustls_pemfile::certs(&mut cert_reader)
        .collect::<Result<Vec<_>, _>>()?;

    // Load private key
    let key_file = File::open(key_path)?;
    let mut key_reader = BufReader::new(key_file);
    let key = rustls_pemfile::private_key(&mut key_reader)?
        .ok_or("No private key found")?;

    let mut config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    // Set ALPN protocols to support HTTP/1.1 and HTTP/2
    config.alpn_protocols = vec![b"http/1.1".to_vec(), b"h2".to_vec()];

    Ok(TlsAcceptor::from(Arc::new(config)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("mock-server")
        .about("OSR Mock Auth Server")
        .subcommand(
            Command::new("auth")
                .about("Run the auth server")
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .value_name("PORT")
                        .help("Port to listen on")
                        .default_value("8080"),
                )
                .arg(
                    Arg::new("cert")
                        .long("cert")
                        .value_name("CERT_FILE")
                        .help("Path to TLS certificate (PEM)"),
                )
                .arg(
                    Arg::new("key")
                        .long("key")
                        .value_name("KEY_FILE")
                        .help("Path to TLS private key (PEM)"),
                )
                .arg(
                    Arg::new("game-server")
                        .long("game-server")
                        .value_name("ADDR")
                        .help("Game server address to return in auth response")
                        .default_value("wss://localhost:9090"),
                ),
        )
        .get_matches();

    if let Some(auth_matches) = matches.subcommand_matches("auth") {
        let port: u16 = auth_matches.get_one::<String>("port").unwrap().parse()?;
        let game_server = auth_matches
            .get_one::<String>("game-server")
            .unwrap()
            .clone();

        let config = AuthServerConfig {
            port,
            game_server_addr: game_server,
            jwt_secret: "mock-secret-key-for-testing".to_string(),
            jwt_ttl_seconds: 3600,
        };

        // Load TLS config if cert and key are provided
        let tls_acceptor = if let (Some(cert), Some(key)) = (
            auth_matches.get_one::<String>("cert"),
            auth_matches.get_one::<String>("key"),
        ) {
            Some(load_tls_config(cert, key)?)
        } else {
            None
        };

        println!(
            "Starting auth server on port {} {}",
            port,
            if tls_acceptor.is_some() {
                "(WSS)"
            } else {
                "(WS - no TLS)"
            }
        );
        run_auth_server(&config, tls_acceptor).await?;
    } else {
        eprintln!("No subcommand provided. Use 'auth' to run the auth server.");
        std::process::exit(1);
    }

    Ok(())
}

async fn run_auth_server(
    config: &AuthServerConfig,
    tls_acceptor: Option<TlsAcceptor>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Auth server listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        let config = config.clone();
        let tls_acceptor = tls_acceptor.clone();

        tokio::spawn(async move {
            if let Some(acceptor) = tls_acceptor {
                // TLS connection
                match acceptor.accept(stream).await {
                    Ok(tls_stream) => {
                        println!("TLS handshake successful");
                        match tokio_tungstenite::accept_async(tls_stream).await {
                            Ok(ws_stream) => {
                                println!("WebSocket connection established (WSS)");
                                handle_connection(ws_stream, &config).await;
                            }
                            Err(e) => {
                                eprintln!("WebSocket handshake failed: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("TLS handshake failed: {}", e);
                    }
                }
            } else {
                // Plain WebSocket connection
                match tokio_tungstenite::accept_async(stream).await {
                    Ok(ws_stream) => {
                        println!("WebSocket connection established (WS)");
                        handle_connection(ws_stream, &config).await;
                    }
                    Err(e) => {
                        eprintln!("WebSocket handshake failed: {}", e);
                    }
                }
            }
        });
    }
}

async fn handle_connection(
    ws_stream: tokio_tungstenite::WebSocketStream<impl tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin>,
    config: &AuthServerConfig,
) {
    use tokio_tungstenite::tungstenite::Message;
    use futures_util::StreamExt;
    use futures_util::SinkExt;

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("Received: {}", text);

                // Parse AuthRequest
                let response = match serde_json::from_str::<AuthRequest>(&text) {
                    Ok(request) => osr_client::auth_server::handle_auth_request(config, &request),
                    Err(e) => {
                        eprintln!("Failed to parse AuthRequest: {}", e);
                        AuthResponse {
                            success: false,
                            token: None,
                            game_server: None,
                            error: Some(osr_client::shared::AuthError::ServerError),
                        }
                    }
                };

                // Send AuthResponse
                let response_json = serde_json::to_string(&response).unwrap();
                if let Err(e) = write.send(Message::Text(response_json)).await {
                    eprintln!("Failed to send response: {}", e);
                    break;
                }
            }
            Ok(Message::Close(_)) => {
                println!("Connection closed");
                break;
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
