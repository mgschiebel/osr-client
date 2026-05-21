# Rust (godot-rs) for Godot client shell

We chose Rust via the godot-rs crate for the osr-client shell (GameState, CameraController, LoadingScreen, etc.) instead of GDScript or C#.

We prioritized (1) distribution size (smallest native binary, no runtime), (2) copyright protection (native compiled code is hardest to reverse-engineer), and (3) multi-platform/mobile support. While GDScript is fastest for MVP iteration, Rust aligns with our ranked priorities. C# was rejected due to limited Godot 4.x mobile export support.

**Considered Options:**

- **GDScript**: Fastest MVP iteration, smallest binary, weakest copyright protection, full mobile support
- **C#**: Good protection, largest binary (.NET runtime), limited Godot 4.x mobile export support
- **Rust (godot-rs)**: Good protection, small native binary, full mobile support, slower compile times
