# OSR Client

A 3D MMORPG client application.

## Domain Glossary

**OSR**:
Old School Revival — a genre of MMORPG inspired by classic 1990s/2000s online RPGs like EverQuest, characterized by slow pacing, lethal combat, heavy exploration, and group-dependent gameplay.
_Avoid_: Old School Renaissance, tabletop OSR

**MVP**:
The first shippable slice of the client: stub auth/char-select screens, an empty room with WASD movement, and toggleable first/third person cameras. No networking, no other players, no chat.
_Avoid_: "phase 1", "initial release"

**Stub screen**:
A placeholder UI that simulates a feature without real logic. Auth and character selection are stubbed in the MVP — they show UI but don't connect to a server.
_Avoid_: mock screen, fake screen

**CameraController**:
A script on the `Player` scene that manages camera zoom via mouse wheel scroll. Scroll in until near-clip-plane collision transitions to first person; scroll out to orbit at a max distance.
_Avoid_: CameraManager, CameraSystem

**NavMesh**:
A navigation mesh on the world scene that defines walkable surfaces. Used by the player movement system to traverse the world.
_Avoid_: navmesh, pathfinding mesh

**LoadingScreen**:
A CanvasLayer 2D scene shown on game start before auth. Displays a static "Loading..." label while assets load, then calls `GameState.goto_auth()`. Reused for scene transitions post-MVP.
_Avoid_: splash screen, startup screen

**Input Map**:
Godot's action-based input system (Project Settings → Input Map). All input is bound to named actions (`move_forward`, `camera_zoom`, etc.) rather than raw keycodes, enabling remapping without code changes.
_Avoid_: input bindings, key mappings

**Shell**:
The generic runtime client that loads game PCKs. Contains GameState, CameraController, LoadingScreen. Written in Rust via godot-rs.
_Avoid_: client, launcher, runtime

**godot-rs**:
The Rust bindings for Godot 4.x. Used to write the client shell in Rust, compiled to a native binary.
_Avoid_: rust-godot, godot-rust

**PCK**:
Godot's packaged format for exporting game content. Game devs export their projects as .pck files, which the client streams and loads via `ProjectSettings.load_resource_pack()`. Supports encryption for copyright protection.
_Avoid_: pack file, resource pack

**godot-rs build**:
Standard workflow: `cargo init --lib` in the Godot project, `godot = "0.4"` in Cargo.toml, build cdylib via `cargo build`, load via `.gdextension` file. A Makefile/justfile wraps the build and copies the binary to `bin/`.
_Avoid_: rust build, build script

**Auth Server**:
The server handling username/password authentication over WSS (WebSocket Secure). Returns a JWT and the game server address. Mock implementation is a Rust binary with `clap` subcommands, containerized via Docker for networking tests.
_Avoid_: login server, auth service

**JWT**:
JSON Web Token — a signed token the auth server issues and the client forwards to the game server as the first WebSocket message after connection. Contains `sub`, `exp`, `iat` claims. Client decodes (not validates) in MVP; validation with cert is a later addition.
_Avoid_: access token, bearer token

**Game Server**:
The server managing the game world. Client connects via WSS with a JWT from the auth server, sent as `{"type": "auth", "token": "eyJ..."}` — the first message after WebSocket open. Sends `enter_world` with `player_id` and `position` after successful handshake.
_Avoid_: world server, game host

**enter_world**:
Game server message sent after successful JWT handshake. Contains `player_id` and `position` for spawning the player character in the world scene.
_Avoid_: world_enter, join_world

**client.toml**:
Configuration file with `auth_server` URL (e.g., `wss://auth.example.com`). Read by GameState on startup. Containerized tests mount a different config pointing at the mock server container.
_Avoid_: config.toml, settings.toml

**Mock Server**:
Containerized Rust binary (same crate, `[[bin]]` target) providing auth and game server stubs. Supports `tc netem` for latency/packet loss simulation and `iptables` for firewall/NAT testing. Docker + docker-compose orchestration.
_Avoid_: fake server, test server
