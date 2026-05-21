# Containerized Mock Servers for Networking Tests

We chose Docker + docker-compose for containerized mock server testing instead of in-process tests, Podman, or separate repos.

We prioritized (1) realistic networking tests (NAT, firewall, latency, packet loss) that require true container separation, (2) `docker-compose` maturity for multi-container orchestration (mock server + optional client container + `tc netem` simulation), and (3) CI/CD ubiquity (every CI provider supports Docker out of the box).

**Considered Options:**

- **In-process mock server in Rust tests**: Fastest TDD loop, no containers, but client and server share the same network namespace. Cannot simulate firewall drops, NAT traversal, or latency. Used for pure Rust unit/integration tests only.
- **Podman + podman-compose**: Daemonless and rootless by default, but `podman-compose` is less mature than `docker-compose` — adds friction to the TDD loop. The `Dockerfile` is written as OCI-compliant (no Docker-specific features) so migration to Podman later is `podman build` / `podman-compose` with minimal changes.
- **Separate repo/workspace member for mock server**: Isolates mock server from client crate, but adds scaffolding overhead and makes shared type extraction (JWT structs, message types) more complex. Rejected in favor of a `[[bin]]` target in the same `rust/` crate.
- **Docker + docker-compose (chosen)**: Mock server is a Rust binary (`main.rs`) with `clap` subcommands (`auth`, `game`). Dockerfile uses multi-stage build. `docker-compose.yml` runs the mock server container with `cap_add: NET_ADMIN` for `iptables`/`tc netem` simulation. E2E tests spawn `docker-compose up`, launch Godot as a subprocess on the host, and parse stdout for scene transition markers.
