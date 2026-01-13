# Implementation Plan - Portable WASM Apps (Pixel Canvas)

**Feature**: Portable WASM Apps (Pixel Canvas)
**Branch**: `001-wasm-pixel-canvas`
**Spec**: [spec.md](./spec.md)
**Status**: In Progress

## Technical Context

- **Language**: Rust
- **Runtime**: Wasmtime (latest stable)
- **Graphics Framework**: SDL2 (`sdl2` crate)
- **Packaging**: Custom binary format (Magic Header + WASM binary)
- **Architecture**:
  - Host App (Rust): Initializes SDL, Wasmtime engine, WASI context.
  - Guest App (WASM): Exports `update(dt)`, `on_resize`, `on_pointer_move`, `on_pointer_down`, `on_pointer_up`, `on_key_down`, `on_key_up`. Imports `update_frame`.
- **Unknowns**:
  - [x] SDL2 vs SDL3 (Decided: SDL2)
  - [x] Wasmtime Memory Access (Decided: Streaming Texture)

## Constitution Check

- **Architecture**:
  - [x] Modular design (Host vs Guest separation)
  - [x] Scalable patterns (Event-driven loop)
- **Code Quality**:
  - [x] Type safety (Rust)
  - [x] Error handling (Result/Option usage for WASM traps/IO)
- **Security**:
  - [x] Sandboxing (WASM/WASI) - Need to explicitly configure WASI capabilities (e.g. limit file access).
- **Performance**:
  - [x] 60FPS Target - Depends on efficient pixel transfer.

## Gates

- [x] **Phase 0 Gate**: Research complete
- [x] **Phase 1 Gate**: Data models and contracts defined
- [ ] **Phase 2 Gate**: Core implementation complete

## Phase 0: Outline & Research

### Goal
Resolve unknowns regarding SDL libraries and Wasmtime integration patterns.

### Tasks

1.  [x] **Research SDL Rust bindings**: Evaluate SDL2 vs SDL3.
2.  [x] **Research Wasmtime Memory Access**: Determine best pattern for reading the `i32* pixels` pointer.
3.  [x] **Research WASI Context**: How to set up minimal WASI in Wasmtime (p1 sync adapter).

### Output
- `research.md`

## Phase 1: Design & Contracts

### Goal
Define the exact binary format, API signatures, and project structure.

### Tasks

1.  [x] **Define Binary Format**: "WAPP" header.
2.  [x] **Define Host/Guest Interface**: `wapps.wit` definitions.
3.  [x] **Create Quickstart**: Host and Guest build instructions.

### Output
- `data-model.md`
- `contracts/`
- `quickstart.md`

## Phase 2: Implementation

### Goal
Build the Host runner and a demo Guest.

### Tasks

1.  [ ] **Scaffold Host Project**: `cargo new host`. Add dependencies (`wasmtime`, `sdl2`, etc.).
2.  [ ] **Implement File Loader**: Code to parse "WAPP" header.
3.  [ ] **Implement WASM Runtime**: Setup Wasmtime Engine, Module, and Linker.
4.  [ ] **Implement Host Functions**: `update_frame` logic.
5.  [ ] **Implement SDL Framework**: Window creation, Event Loop.
6.  [ ] **Integrate Run Loop**: Call `update(dt)` in guest.
7.  [ ] **Implement Input/Resize forwarding**: Map SDL events.
8.  [ ] **Build Demo Guest**: A pure Rust/WASM Conway's Game of Life simulation.
9.  [ ] **Verification**: Run demo app, resize window, check input.

### Output
- Working Rust Source Code
- Demo `.wapp` file
