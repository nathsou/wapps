# Tasks: Portable WASM Apps (Pixel Canvas)

**Feature**: Portable WASM Apps
**Branch**: `001-wasm-pixel-canvas`
**Status**: Complete

## Phase 1: Setup
**Goal**: Initialize project structure and dependencies.

- [X] T001 Initialize workspace with `host` crate and `examples/demo` directory in `Cargo.toml`
- [X] T002 Add dependencies to `host/Cargo.toml`: `sdl2`, `wasmtime`, `wasmtime-wasi`, `anyhow`, `clap`
- [X] T003 Setup basic logging (env_logger) and error handling in `host/src/main.rs`

## Phase 2: Foundational
**Goal**: Core infrastructure (Loading & Runtime Setup) - Blocking for US1.

- [X] T004 Implement WAPP File Parsing and Validation (Read Header, Verify Magic/Version) in `host/src/loader.rs`
- [X] T004b Implement error handling for invalid WAPP headers (invalid magic/version) with user-friendly error messages
- [X] T005 [P] Setup Wasmtime Engine, Linker, WASI Context (Sync), and configure trap handler for graceful error reporting in `host/src/runtime.rs`
- [X] T005b Configure WASI security policy: enable minimal WASI (clock, random, stdio only), disable file system access in `host/src/runtime.rs`

## Phase 3: User Story 2 - Develop Compliant WASM App (P1)
**Goal**: Create a valid `.wapp` package to verify the host.
**Story**: [US2] Develop Compliant WASM App

- [X] T006 [US2] Create Guest Rust project structure in `examples/demo/Cargo.toml` (`cdylib` target)
- [X] T007 [P] [US2] Define `wapps` host imports (`update_frame`) in `examples/demo/src/lib.rs`
- [X] T008 [P] [US2] Implement Guest Logic (Game of Life simulation, Export `update`, `on_resize`) in `examples/demo/src/lib.rs`
- [X] T009 [US2] Create packaging script `examples/demo/package.sh` to compile WASM and prepend 5-byte header

## Phase 4: User Story 1 - Run Portable WASM App (P1)
**Goal**: The Host Application runs the Guest Package.
**Story**: [US1] Run Portable WASM App

- [X] T010 [US1] Initialize SDL2 Video Subsystem and create Window in `host/src/graphics.rs`
- [X] T011 [US1] Implement Main Event Loop (Time step, Event Pump) in `host/src/main.rs`
- [X] T012 [P] [US1] Define Host Interface struct/functions (store `i32*`, `width`, `height` from guest) in `host/src/host_interface.rs`
- [X] T013 [US1] Link `wapps::update_frame` import to Wasmtime Linker in `host/src/runtime.rs`
- [X] T014 [US1] Implement Texture Update Logic (Read WASM Memory -> Update SDL Texture) in `host/src/graphics.rs`
- [X] T015 [US1] Call Guest export `update(dt)` from Main Loop in `host/src/main.rs`
- [X] T016 [US1] Implement `on_resize` handling (SDL WindowEvent::Resized -> Guest Export) in `host/src/main.rs`
- [X] T017 [US1] Implement Input forwarding (SDL Mouse/Keyboard -> Guest `on_pointer_`/`on_key_`) in `host/src/main.rs`

## Phase 5: Polish
**Goal**: Final integration and UX cleanups.

- [X] T018 Wire up CLI argument parsing to load `.wapp` file specified by user in `host/src/main.rs`
- [X] T019 Verify support for Window Resizing (Snapping to guest content size) in `host/src/main.rs`
- [X] T020 Manual verification: Test invalid WAPP files, WASM traps, and error message clarity
- [X] T021 Run `cargo fmt` and `cargo clippy` across workspace

## Dependencies

- **US2 (Guest)** depends on Spec Contracts.
- **US1 (Host)** depends on **Foundational** and **Setup**.
- **US1 Verification** depends on **US2** (Need a valid package to run).

## Parallel Execution Examples

- **Developer A**: T006, T007, T008, T009 (Building the Guest App)
- **Developer B**: T010, T011, T012, T013 (Building the Host Runner)

## Implementation Strategy

1. **MVP**: Build Host that can just load a WASM and run a loop, printing "tick". Build Guest that exports empty update.
2. **Graphics**: Connect `update_frame` to SDL.
3. **Interactive**: Connect Input events.
