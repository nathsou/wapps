import { WASI, File, OpenFile, ConsoleStdout } from 'https://esm.sh/@bjorn3/browser_wasi_shim@0.4.2';

export class WappRuntime {
    constructor(canvas) {
        this.canvas = canvas;
        this.ctx = canvas.getContext('2d');
        this.wasi = null;
        this.instance = null;
        this.memory = null;
        this.width = 0;
        this.height = 0;
        this.frameBufferPtr = 0;
        this.pixelsView = null;
    }

    async load(bytes){
        // Parse WAPP Header
        // Struct: Magic(4) | Version(4) | JSON_Len(4) | JSON(N) | WASM(...)
        const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
        
        // 1. Magic: WAPP (0x57415050)
        const magic = view.getUint32(0, false); // Big Endian check for string
        if (magic !== 0x57415050) {
            throw new Error("Invalid magic header: Not a WAPP file");
        }

        // 2. Version: u32 LE
        const version = view.getUint32(4, true);
        if (version !== 1) {
            throw new Error(`Unsupported WAPP version: ${version}`);
        }

        // 3. JSON Length: u32 LE
        const jsonLen = view.getUint32(8, true);
        
        // 4. JSON Metadata
        const jsonBytes = bytes.subarray(12, 12 + jsonLen);
        const decoder = new TextDecoder('utf-8');
        let metadata = {};
        try {
            const jsonString = decoder.decode(jsonBytes);
            metadata = JSON.parse(jsonString);
        } catch (e) {
            throw new Error("Failed to parse WAPP metadata: " + e.message);
        }

        // 5. WASM Payload
        const wasmBytes = bytes.subarray(12 + jsonLen);

        const args = [];
        const env = [];
        const fds = [
            new OpenFile(new File([])), // stdin
            new ConsoleStdout(console.log), // stdout
            new ConsoleStdout(console.error), // stderr
        ];

        this.wasi = new WASI(args, env, fds);

        const wappsImports = {
            wapps: {
                update_frame: (width, height, ptr) => {
                    this.frameBufferPtr = ptr;
                    this.width = width;
                    this.height = height;
                }
            }
        };

        const wasm = await WebAssembly.instantiate(wasmBytes, {
            "wasi_snapshot_preview1": this.wasi.wasiImport,
            ...wappsImports
        });

        this.instance = wasm.instance;
        this.memory = this.instance.exports.memory;
        
        // Initialize WASI (runs _start if present)
        if (this.instance.exports._start) {
             this.wasi.start(this.instance);
        } else {
             this.wasi.initialize(this.instance); // If reactor mode supported by shim
        }

        return metadata;
    }

    start() {
        let lastTime = performance.now();
        const loop = (time) => {
            const dt = (time - lastTime) / 1000;
            lastTime = time;

            if (this.instance) {
                // Call WAPP update
                this.instance.exports.update(dt);
                this.render();
            }
            requestAnimationFrame(loop);
        };
        requestAnimationFrame(loop);
    }

    render() {
        if (!this.frameBufferPtr || !this.width || !this.height) return;

        const size = this.width * this.height * 4;

        // Cache view to avoid allocation if memory hasn't grown/moved
        if (!this.pixelsView || 
            this.pixelsView.buffer !== this.memory.buffer || 
            this.pixelsView.byteOffset !== this.frameBufferPtr ||
            this.pixelsView.byteLength !== size) {
            
            this.pixelsView = new Uint8ClampedArray(this.memory.buffer, this.frameBufferPtr, size);
        }
        
        // We might need to scale the canvas or just put image data
        if (this.canvas.width !== this.width || this.canvas.height !== this.height) {
            this.canvas.width = this.width;
            this.canvas.height = this.height;
        }

        const imageData = new ImageData(this.pixelsView, this.width, this.height);
        this.ctx.putImageData(imageData, 0, 0);
    }

    // Input Handling
    handleMouseDown(x, y, button) {
        if (this.instance?.exports.on_pointer_down) {
            this.instance.exports.on_pointer_down(x, y, button);
        }
    }

    handleMouseUp(x, y, button) {
        if (this.instance?.exports.on_pointer_up) {
            this.instance.exports.on_pointer_up(x, y, button);
        }
    }

    handleMouseMove(x, y) {
        if (this.instance?.exports.on_pointer_move) {
            this.instance.exports.on_pointer_move(x, y);
        }
    }

    handleKeyDown(code) {
        if (this.instance?.exports.on_key_down) {
            this.instance.exports.on_key_down(code);
        }
    }

    handleKeyUp(code) {
        if (this.instance?.exports.on_key_up) {
            this.instance.exports.on_key_up(code);
        }
    }
}
