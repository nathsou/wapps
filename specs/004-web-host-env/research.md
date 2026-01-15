# Research: WAPP Web Host Environment

**Status**: Complete
**Date**: 2026-01-15

## 1. Technical Architecture

### 1.1 Host Execution Environment
The user requirement has shifted to a **Pure Web** implementation.
- **Decision**: The entire host runtime will be written in JavaScript/TypeScript. NO host-side Rust compilation.
- **Rationale**: Simplifies the build chain (no cargo/wasm-pack), reduces download size (no double-wasm), and leverages modern browser capabilities directly.

### 1.2 WASI Support
WAPP binaries are compiled to `wasm32-wasip1` and need WASI exports.
- **Solution**: Use [`@bjorn3/browser_wasi_shim`](https://github.com/bjorn3/browser_wasi_shim).
- **Implementation**: Import the shim, create a `WASI` instance with minimal configuration (stdout/stderr piped to console), and pass `wasi.wasiImport` to the `WebAssembly.instantiate` call.

### 1.3 Canvas & Rendering
- **Memory Access**: The WAPP instance exports `memory`. In JS, we can create a `Uint8ClampedArray` view on `exports.memory.buffer`.
- **Pixel Transfer**: The `update_frame(w, h, ptr)` export from the guest gives us the offset. We read `new Uint8ClampedArray(memory.buffer, ptr, w * h * 4)` and put it into an `ImageData` object to draw on the canvas.

## 2. Dependencies

- **Runtime & Launcher**:
    - **NPM Package**: `@bjorn3/browser_wasi_shim` (for WASI).
    - **Development**: A simple HTTP server (Python or Node) to serve the static files.
    - **No Bundler (MVP)**: Use ES modules directly in the browser if possible, or a lightweight bundler (e.g., `esbuild` or just import maps) if specific npm packages require it. *Correction*: `browser_wasi_shim` is distributed as a module, so an import map or simple bundler might be needed if not using a CDN. For simplicity/robustness, we might use a CDN link or a simple `npm install` + copy script.

## 3. Deployment Strategy

- **Build**:
    - Minimal build step: Copy assets to dist folder.
- **Hosting**:
    - GitHub Pages.
- **CI**:
    - `actions/checkout`
    - Copy files.
    - `actions/deploy-pages`.

## 4. Unknowns Resolved

- **Host Interface**: Implemented as a JS class `HostInterface` that maintains the `canvas` context and `ImageData`.
- **Event Loop**: JS `requestAnimationFrame` calls `wapp_instance.exports.update(dt)`.
