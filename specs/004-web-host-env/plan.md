# Implementation Plan - WAPP Web Host Environment

**Feature**: WAPP Web Host Environment
**Status**: Planning
**Spec**: [spec.md](spec.md)

## Technical Context

The goal is to bring the WAPP runtime to the web, adhering to a strict **"No Rust"** constraint for the host implementation. The web host will be a Pure JavaScript application that uses `@bjorn3/browser_wasi_shim` to execute WAPP binaries (which are WASM with WASI dependencies).

### Architecture
- **Web Runtime (`web-runtime`)**: A JavaScript module that wraps the WASM instantiation, integrates `browser_wasi_shim` for system calls, and implements the custom `wapps` host interface (graphics/input) directly in JS.
- **Launcher App (`web-launcher`)**: A simple HTML/CSS/JS frontend that provides the user interface (Canvas, File Picker) and consumes the `web-runtime`.
- **WASI Shim**: `@bjorn3/browser_wasi_shim` provides the `fd_write`, `random_get`, `clock_time_get` implementation required by the WAPP binaries.

### Technology Stack
- **Language**: JavaScript (ES Modules).
- **Core Library**: [`@bjorn3/browser_wasi_shim`](https://github.com/bjorn3/browser_wasi_shim).
- **Rendering**: HTML5 Canvas API (2D Context with `putImageData`).
- **Build System**: None desired (Native ES Modules), but may need a simple bundler or import map if using npm packages. For simplicity/constitution alignment, we will try to use ES modules directly served or a minimal script to copy dependencies.
- **Hosted**: GitHub Pages.

### Constraints & Unknowns
- **Shim Integration**: Need to ensure the `imports` object passed to `WebAssembly.instantiate` correctly combines `wasi_snapshot_preview1` (from shim) and `wapps` (our custom host imports).
- **Memory Access**: JS code needs to access the WASM memory buffer to read pixels. `WebAssembly.Memory` buffer detaches on grow; need to handle buffer updates safely.

## Constitution Check

- [x] **Safe**: Execution is confined to the browser sandbox.
- [x] **Secure**: No server-side components.
- [x] **Performant**: Direct JS memory access to WAPP linear memory is efficient.
- [x] **Maintainable**: Avoiding a Rust-WASM toolchain for the host reduces complexity significantly for this use case.

## Phase 0: Research & Prototypes

**Goal**: Validate the JS-only approach and shim compatibility.

- [ ] Prototype `wasi-shim-test`: Create a small HTML file that imports the shim and tries to run a dummy WASM file to verify `wasi_snapshot_preview1` linking.
- [ ] Determine how to vend `@bjorn3/browser_wasi_shim` without a complex bundler (e.g. download a standalone ESM build or use a CDN for dev, vendor for prod).

## Phase 1: Design & Contracts

**Goal**: Define the JS structure.

- [ ] Refine `contracts/web-runtime-api.md`: Ensure the class structure matches a pure JS implementation (constructor, async `load`, public methods).
- [ ] data-model.md: Update to reflect JS-side state management (no Rust structs).

## Phase 2: Implementation

**Goal**: Build the runtime and launcher.

- [ ] **Step 1: Setup**: Create `host/web` directory structure. Vendor the `browser_wasi_shim`.
- [ ] **Step 2: Runtime Logic**: Implement `host/web/runtime.js` which handles:
    - `HostInterface`: Storing frame dimensions and buffer pointer.
    - `imports`: Creating the import object with `wapps.update_frame` and shim exports.
    - `instantiate`: Loading the WASM.
- [ ] **Step 3: Launcher UI**: Implement `host/web/index.html` and `host/web/main.js` to handle File Drag/Drop and generic UI.
- [ ] **Step 4: Input/Output**: Wire up `requestAnimationFrame` to call `wapp.exports.update` and render to Canvas. Wire mouse/keyboard events to `wapp.exports.on_*`.

## Phase 3: Deployment

- [ ] `web-host-deploy.yml`: Create a GitHub Action to deploy `host/web` to GitHub Pages.