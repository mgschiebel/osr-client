# Human E2E Test Plan — Networking Slice

## Prerequisites

- `just build` completes successfully (Rust crate compiles with networking modules)
- `cargo run --bin mock-server -- auth --port 8080` starts the mock auth server
- `cargo run --bin mock-server -- game --port 8081` starts the mock game server
- `client.toml` exists with `auth_server = "wss://localhost:8080"` (or equivalent)
- Godot 4.3 installed and `godot` is on PATH

## Test Cases

### TC-01: Auth Success → CharacterSelect → World Entry

1. Launch Godot: `just run`
2. Verify LoadingScreen appears with "Loading..." label
3. Verify LoadingScreen auto-transitions to AuthScreen
4. Enter username: `testuser`
5. Enter password: `testpass`
6. Click "Enter"
7. Verify LoadingScreen appears with progress messages (e.g., "Connecting to game server...")
8. Verify CharacterSelectScreen appears
9. Click "Start"
10. Verify LoadingScreen appears with progress messages
11. Verify EmptyRoom loads with capsule mesh at the server-specified position
12. Verify WASD movement works via NavMesh
13. Verify mouse wheel zooms camera (first ↔ third person)

### TC-02: Auth Failure — Invalid Credentials

1. Launch Godot: `just run`
2. Navigate to AuthScreen
3. Enter username: `wronguser`
4. Enter password: `wrongpass`
5. Click "Enter"
6. Verify red error label appears: "Invalid credentials"
7. Verify password field is cleared
8. Verify still on AuthScreen (no transition)

### TC-03: Auth Failure — Server Unreachable

1. Stop the mock auth server
2. Launch Godot: `just run`
3. Navigate to AuthScreen
4. Enter valid credentials
5. Click "Enter"
6. Verify error message: "Unable to reach server" or "Connection failed"
7. Verify still on AuthScreen

### TC-04: Game Server Connection Failure

1. Start mock auth server only (no game server)
2. Launch Godot, auth with valid credentials
3. Click "Start" on CharacterSelect
4. Verify LoadingScreen shows error: "Unable to connect to game server"
5. Verify returns to CharacterSelectScreen (or stays on LoadingScreen with error)

### TC-05: Containerized Test — Latency Simulation

1. Start mock servers via docker-compose: `docker-compose up`
2. Apply latency: `docker exec mock-server tc qdisc add dev eth0 root netem delay 500ms`
3. Launch Godot, auth with valid credentials
4. Verify LoadingScreen shows progress messages with noticeable delay
5. Verify auth eventually succeeds and CharacterSelect appears
6. Clean up: `docker-compose down`

### TC-06: Containerized Test — Packet Loss

1. Start mock servers via docker-compose
2. Apply packet loss: `docker exec mock-server tc qdisc add dev eth0 root netem loss 30%`
3. Launch Godot, auth with valid credentials
4. Verify auth succeeds (with possible retries)
5. Verify world entry succeeds
6. Clean up: `docker-compose down`

### TC-07: JWT Expiry Handling

1. Configure mock auth server to issue 5-second TTL tokens (or edit mock source)
2. Launch Godot, auth with valid credentials
3. Wait 6+ seconds on CharacterSelect
4. Click "Start"
5. Verify client detects expired token, returns to AuthScreen (or shows re-auth prompt)
6. Re-auth and verify world entry works with fresh token

## Notes

- All server addresses and credentials are configurable via `client.toml` and mock server flags
- Progress messages on LoadingScreen should be visible for at least 1 second (not flash and disappear)
- Error messages should persist until the user takes action (no auto-dismiss)
- Containerized tests require `cap_add: NET_ADMIN` in docker-compose.yml for `tc netem`
