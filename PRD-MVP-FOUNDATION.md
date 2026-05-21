## Problem Statement

The osr-client project has an empty repository with only a placeholder README and CONTEXT.md. There is no usable client application, no project structure, no build system, and no way to run or test anything. Developers who want to build an OSR-style MMORPG client have no foundation to build from.

## Solution

Build an MVP foundation for the osr-client: a Godot 4.3 project with a Rust/godot-rs shell that provides stub screens (auth, character select), an empty room with WASD navigation via NavMesh, and a scroll-wheel zoom-to-toggle camera system (first person ↔ third person). The project will be structured as a generic shell that loads game PCKs, with a build system via justfile, proper directory layout, and Input Map configuration.

## User Stories

1. As a developer, I want a Godot 4.3 project initialized with proper directory structure, so that I have a working foundation to build upon.

2. As a developer, I want a Rust/godot-rs library crate configured with a .gdextension file, so that I can write the client shell in Rust.

3. As a developer, I want a justfile with `build` and `run` targets, so that I can compile the Rust crate and launch Godot with one command.

4. As a player, I want to see a LoadingScreen with "Loading..." when I launch the game, so that I know the game is starting while assets initialize.

5. As a player, I want the LoadingScreen to automatically transition to the AuthScreen, so that I can begin the login flow without manual interaction.

6. As a player, I want an AuthScreen with username and password fields and an "Enter" button, so that I can simulate logging in (stubbed, no real auth).

7. As a player, I want clicking "Enter" on the AuthScreen to transition to the CharacterSelectScreen, so that I can select a character.

8. As a player, I want a CharacterSelectScreen with a "Start" button, so that I can simulate entering the game world (stubbed, no real character data).

9. As a player, I want clicking "Start" on the CharacterSelectScreen to load the EmptyRoom world, so that I can explore the game world.

10. As a player, I want to move my character with WASD keys in the EmptyRoom, so that I can explore the world.

11. As a player, I want WASD movement to use NavMesh pathfinding, so that my character walks on valid surfaces and respects the navigation mesh.

12. As a player, I want to see a capsule mesh representing my character in the EmptyRoom, so that I have visual feedback of my position.

13. As a player, I want to scroll the mouse wheel to zoom the camera, so that I can adjust my view distance.

14. As a player, I want the camera to orbit around my character at third person distance when zoomed out, so that I can see my character and the surrounding area.

15. As a player, I want the camera to switch to first person when I scroll in past the near-clip-plane collision point, so that I can see from my character's viewpoint.

16. As a player, I want the camera to switch back to third person orbit when I scroll out past the threshold, so that I can return to the over-the-shoulder view.

17. As a developer, I want all input (WASD, mouse wheel, etc.) defined in Godot's Input Map with named actions, so that inputs are remappable without code changes.

18. As a developer, I want a GameState singleton (Rust autoload) managing scene transitions, so that screen flow is centralized and testable.

19. As a player, I want to see a floor plane, skybox, and lighting in the EmptyRoom, so that the world feels like a 3D space.

20. As a developer, I want the Player scene to use CharacterBody3D with CollisionShape3D, MeshInstance3D, Camera3D, and NavigationAgent3D, so that the player has physics, visuals, camera, and pathfinding.

21. As a developer, I want the CameraController logic written in Rust, so that the shell benefits from Rust's performance and copyright protection.

22. As a developer, I want a LoadingScreen scene that can be reused for post-MVP scene transitions, so that I have a consistent loading experience.

23. As a game dev using osr-client, I want the client to be a generic shell that can load my game's PCK file, so that I can stream my entire game to the client.

24. As a game dev using osr-client, I want PCK encryption support, so that my game content is protected from reverse engineering.

## Implementation Decisions

- **Godot version**: 4.3 stable
- **Shell language**: Rust via godot-rs 0.4+ (recorded in ADR-0001)
- **Project structure**:
  - `scenes/auth/AuthScreen.tscn`
  - `scenes/character_select/CharacterSelectScreen.tscn`
  - `scenes/world/EmptyRoom.tscn`
  - `scenes/shared/Player.tscn`
  - `scripts/autoloads/GameState.gd` → Rust autoload via .gdextension
  - `scripts/controllers/CameraController.gd` → Rust script via godot-rs
  - `assets/` for textures, models, etc.
  - `ui/themes/` for UI theming
- **Build system**: justfile with `build` (cargo build + copy to `bin/`) and `run` (launch Godot)
- **No third-party Godot libraries**: all features written in-project
- **Input Map actions**: `move_forward`, `move_back`, `move_left`, `move_right`, `camera_zoom` — all bound in Project Settings
- **Player scene**: CharacterBody3D root with CollisionShape3D (capsule), MeshInstance3D (capsule mesh), Camera3D, NavigationAgent3D, and CameraController script
- **CameraController**: mouse wheel scroll zooms camera in/out; transition to first person occurs when camera would collide with near clip plane; third person orbits at max distance
- **GameState**: Rust autoload singleton managing scene transitions: LoadingScreen → AuthScreen → CharacterSelectScreen → EmptyRoom
- **Stub screens**: AuthScreen and CharacterSelectScreen show UI but have no real backend logic — buttons call GameState methods directly
- **LoadingScreen**: CanvasLayer 2D scene, "Loading..." label, auto-transitions to AuthScreen after short delay (simulating asset load)
- **EmptyRoom**: floor plane (with NavMesh), skybox, DirectionalLight3D, Player scene instance
- **No networking for MVP**: no mock server, no PCK streaming, no other players, no chat — all deferred to post-MVP
- **No data model for MVP**: GameState tracks current scene name only; no save files, no player stats, no inventory
- **Testing**: Rust unit tests for GameState transition logic and CameraController zoom math; manual verification for Godot integration (scene rendering, input response)
- **Binary handling**: compiled Rust cdylib (.so/.dll/.dylib) gitignored; developers build from source
- **PCK streaming architecture**: post-MVP vertical; MVP establishes the shell pattern only

## Testing Decisions

- **GameState**: Unit test scene transition logic (auth → char-select → world) as pure Rust functions. Test that invalid transitions are rejected. Prior art: standard Rust `#[test]` functions.
- **CameraController**: Unit test zoom math — verify first/third person transition points, near-clip collision detection, max distance clamping. Prior art: standard Rust `#[test]` functions.
- **Godot integration**: Manual verification against MVP success criteria (see below). No automated integration tests for MVP.
- **Good test criteria**: Test public interfaces (GameState transitions, CameraController zoom output) not implementation details (internal state mutations).

**MVP Success Criteria (manual verification):**

1. Game launches → shows LoadingScreen → transitions to AuthScreen automatically
2. AuthScreen has username/password fields + "Enter" button → clicking it transitions to CharacterSelectScreen
3. CharacterSelectScreen has "Start" button → clicking it loads EmptyRoom
4. In EmptyRoom: WASD moves the capsule mesh via NavMesh pathfinding
5. Mouse wheel scroll: zoom out = third person orbit, zoom in past clip = first person
6. EmptyRoom has visible floor plane, skybox, and DirectionalLight3D
7. No crashes, no errors in Godot output

## Out of Scope

- Real authentication or connection to a game server
- Character creation or character data persistence
- Other players rendered in the world
- Chat system
- Combat, inventory, NPCs, quests, spellcasting
- PCK streaming / asset streaming (post-MVP vertical)
- Mock server or network mocking
- Mobile export testing (architecture supports it, but no MVP testing)
- CI/CD pipeline
- Modding support (architecture supports it via PCK + user:// GDScript, but not implemented)
- C# or GDScript in the shell (Rust only per ADR-0001)

## Further Notes

- The client is designed as a **generic shell** for multiple OSR-style games — game devs export their game as a PCK and the client streams it
- godot-rs compile times will be noticeable but MVP scope is small enough to be manageable
- The NavMesh in EmptyRoom should be basic but demonstrate the framework for future complex worlds
- Long-term goal: single client that lets users pick which independent game to stream and play
- Asset streaming for massive game worlds is a future vertical sprint (custom streaming design)
