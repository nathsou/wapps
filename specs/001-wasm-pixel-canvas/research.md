# Research: Portable WASM Apps (Pixel Canvas)

**Feature**: `001-wasm-pixel-canvas`
**Date**: 2026-01-13
**Status**: Complete

## Technology Stack Decisions

### 1. Graphics Framework: SDL2
**Choice**: `sdl2` (Rust crate)
**Status**: Mature and Stable.
**Rationale**:
- User explicitly requested change to SDL2 for stability/suitability.
- `sdl2` crate is the standard for Rust graphics apps.
- **Strategy**: Use standard `sdl2` crate with `Window` and `Canvas` or `TextureCreator`.

### 2. WASM Runtime: Wasmtime + WASI
**Choice**: Wasmtime v20+ with `wasmtime-wasi::p1`
**Rationale**:
- Need synchronous execution for the game loop.
- `wasmtime-wasi` has moved to async/Component Model by default.
- **Strategy**: Use `wasmtime_wasi::p1::add_to_linker_sync` to link legacy WASI (Preview 1) functions to the synchronous `Store`. This avoids the complexity of async Rust in the main loop while supporting standard WASI modules (which mostly target Preview 1 today).

### 3. Memory Transfer
**Choice**: Single-copy Texture Update
**Rationale**:
- "Zero-copy" from WASM memory to GPU is generally not possible with standard SDL hardware textures (requires upload).
- **Strategy**:
  1. Acquire slice of WASM linear memory: `memory.data(&store)[ptr..ptr+len]`.
  2. Use `SDL_Texture::update` (or `lock`/`unlock` for Streaming) to copy bytes to GPU VRAM.
  3. Use `SDL_TEXTUREACCESS_STREAMING` for the texture.

### 4. Binary Format
**Choice**: Concatenated Header
**Structure**:
```
[0..4]   "WAPP" (Magic)
[4]      0x01   (Version)
[5..]    WASM Binary
```
**Rationale**: Simple, zero-overhead parsing (just slice the file).

## Unknowns Resolved

- **SDL2** -> Use standard `sdl2` crate.
- **Wasmtime WASI** -> Use `p1` sync adapter.
- **Memory** -> Streaming texture update.
