# Web Runtime JS API Contract

**Version**: 1.0.0
**Status**: Draft

The `wapps-web-runtime` module exports the following interface for consuming applications (the launcher).

## Shared Types

```typescript
// WASM instantiation result
export type InitOutput = WebAssembly.Exports;

// Input event types matching wapps-host behavior
export enum InputType {
    MouseMove = 0,
    MouseDown = 1,
    MouseUp = 2,
    KeyDown = 3,
    KeyUp = 4,
}
```

## Main Class: `WappRuntime`

The primary entry point for managing a WAPP session.

```typescript
export class WappRuntime {
    /**
     * Initialize the runtime and bind to a canvas element.
     * @param canvasId The DOM ID of the HTMLCanvasElement to render to.
     */
    constructor(canvasId: string);

    /**
     * Load and run a WAPP binary.
     * Starts the animation loop effectively immediately.
     * 
     * @param wasmBytes The raw bytes of the .wapp file (WASM module).
     * @throws Error if the WASM is invalid or missing imports.
     */
    runWapp(wasmBytes: Uint8Array): Promise<void>;

    /**
     * Send a keyboard event to the WAPP.
     * @param keyCode The micro-key-code (scancode).
     * @param isDown True for KeyDown, False for KeyUp.
     */
    handleKeyEvent(keyCode: number, isDown: boolean): void;

    /**
     * Send a mouse event to the WAPP.
     * @param x X coordinate (relative to canvas).
     * @param y Y coordinate (relative to canvas).
     * @param buttons Mouse button mask (or button index).
     * @param isDown True for Down, False for Up/Move.
     */
    handleMouseEvent(x: number, y: number, buttons: number, isDown: boolean): void;
    
    /**
     * Handle resize events.
     * Should be called when the canvas size changes.
     */
    handleResize(width: number, height: number): void;
}
```

## Usage Example

```javascript
import init, { WappRuntime } from './wapps_web_runtime.js';

async function main() {
    await init();
    
    const runtime = new WappRuntime("wapp-canvas");
    
    // Load file
    const response = await fetch("demo.wapp");
    const buffer = await response.arrayBuffer();
    
    // Run
    await runtime.runWapp(new Uint8Array(buffer));
}
```
