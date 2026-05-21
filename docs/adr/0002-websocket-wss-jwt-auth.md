# WebSocket (WSS) + JWT for Client Auth

We chose WebSocket with TLS (WSS) and JWT tokens for the client auth flow instead of raw TCP, gRPC, or ENet.

We prioritized (1) protocol simplicity for request/response auth, (2) forward compatibility with web exports (WSS works in browsers), and (3) mature Rust crate ecosystem (`tokio-tungstenite`, `rustls`, `jsonwebtoken`).

**Considered Options:**

- **Raw TCP + custom framing**: Lowest latency, but requires building framing, message correlation, and reconnection from scratch. Overkill for a simple auth handshake.
- **gRPC/HTTP2**: Structured RPC with great tooling, but heavier weight and less common for game client auth flows. No streaming advantage for a one-shot auth.
- **WebSocket (WS/WSS)**: HTTP-upgraded persistent connection, works in browsers for future web export, well-supported in Rust via `tokio-tungstenite`. Auth is request/response; game server later may switch to ENet/QUIC for real-time updates.
- **ENet/QUIC**: UDP-based, designed for real-time game state updates with packet loss handling. Deferred for post-auth game state streaming where latency matters.

JWT tokens are decoded client-side (claims read, no signature validation in MVP) with a structured path to add `validate_token()` later. Mock server issues short-lived HS256-signed JWTs using a self-signed cert. Client forwards the JWT to the game server as the first WebSocket message after connection (`{"type": "auth", "token": "eyJ..."}`).
