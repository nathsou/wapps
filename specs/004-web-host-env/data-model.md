# Data Model: Web Host Environment

## Entities

### Web Host Runtime (`WappRuntime`)
Manages the lifecycle of the WAPP application inside the browser.
- **State**:
    - `canvas`: Reference to the DOM element.
    - `instance`: The WebAssembly Instance of the running WAPP.
    - `memory`: The WebAssembly Memory of the running WAPP.
    - `frame_buffer_ptr`: Pointer to the pixel buffer in WAPP memory.
    - `host_interface`: Stores the latest frame details (width, height).

### WAPP Instance
The instantiated WebAssembly module.
- **Imports**:
    - `wapps`: `update_frame(width, height, ptr)`
    - `wasi_snapshot_preview1`: `fd_write`, `random_get`, `clock_time_get`, etc.
- **Exports**:
    - `memory`: Linear memory.
    - `update(dt)`: Main loop function.
    - `on_resize(w, h)`: Event handler.
    - `on_pointer_*`: Input handlers.
    - `on_key_*`: Input handlers.

## Memory flow

1. **Guest Update**:
   When `update(dt)` is called, the guest writes RGBA pixels to its own linear memory.
   
2. **Host Update Frame**:
   The guest calls import `wapps::update_frame(w, h, ptr)`.
   The host (Runtime) stores `ptr`, `w`, and `h`. It does **not** copy the data immediately to avoid double copying if possible, OR it copies it to a safe staging area if asynchronous rendering is needed.
   *Optimization*: The JS rendering loop reads from `memory.buffer` at `ptr` directly during the `draw` phase to creating the `ImageData`.

3. **Rendering**:
   `requestAnimationFrame` loop calls the helper to copy `memory[ptr..len]` to the Canvas context.
