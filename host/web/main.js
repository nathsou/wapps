// Main entry point
import { WappRuntime } from './runtime.js';

const canvas = document.getElementById('wapp-canvas');
const runtime = new WappRuntime(canvas);
const overlay = document.getElementById('overlay');
const filePicker = document.getElementById('file-picker');
const errorMessage = document.getElementById('error-message');

async function loadFile(file) {
    if (!file) return;
    
    console.log(`Loading ${file.name}...`);
    // Hide overlay and error
    overlay.classList.add('hidden');
    errorMessage.classList.add('hidden');
    errorMessage.textContent = '';
    
    try {
        const buffer = await file.arrayBuffer();
        await runtime.load(new Uint8Array(buffer));
        runtime.start();
    } catch (e) {
        console.error("Failed to load WAPP:", e);
        errorMessage.textContent = "Failed to load WAPP: " + e.message;
        errorMessage.classList.remove('hidden');
        overlay.classList.remove('hidden');
    }
}

// File Picker
filePicker.addEventListener('change', (e) => {
    loadFile(e.target.files[0]);
});

// Drag & Drop
document.addEventListener('dragover', (e) => {
    e.preventDefault();
});

document.addEventListener('drop', (e) => {
    e.preventDefault();
    if (e.dataTransfer.files.length > 0) {
        loadFile(e.dataTransfer.files[0]);
    }
});

// Input Handling
canvas.addEventListener('mousedown', (e) => {
    const x = e.offsetX * (canvas.width / canvas.clientWidth);
    const y = e.offsetY * (canvas.height / canvas.clientHeight);
    // Button: 0->1 (Left), 1->2 (Middle), 2->3 (Right)
    runtime.handleMouseDown(x, y, e.button + 1);
});

canvas.addEventListener('mouseup', (e) => {
    const x = e.offsetX * (canvas.width / canvas.clientWidth);
    const y = e.offsetY * (canvas.height / canvas.clientHeight);
    runtime.handleMouseUp(x, y, e.button + 1);
});

canvas.addEventListener('mousemove', (e) => {
    const x = e.offsetX * (canvas.width / canvas.clientWidth);
    const y = e.offsetY * (canvas.height / canvas.clientHeight);
    runtime.handleMouseMove(x, y);
});

// Key Mapping (Partial USB HID)
const KEY_MAP = {
    "Space": 44,
    "KeyR": 21,
    "KeyC": 6,
    // Add more as needed
    "ArrowUp": 82, "ArrowDown": 81, "ArrowLeft": 80, "ArrowRight": 79,
    "Enter": 40, "Escape": 41, "Backspace": 42, "Tab": 43
};

window.addEventListener('keydown', (e) => {
    if (KEY_MAP[e.code]) {
        runtime.handleKeyDown(KEY_MAP[e.code]);
    }
});

window.addEventListener('keyup', (e) => {
    if (KEY_MAP[e.code]) {
        runtime.handleKeyUp(KEY_MAP[e.code]);
    }
});

console.log('WAPP Host initialized');
