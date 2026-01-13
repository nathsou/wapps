# Data Model: Portable WASM Apps

**Feature**: `001-wasm-pixel-canvas`
**Version**: 1.0

## Binary File Format (`.wapp`)

The application package is a single binary file with a custom header followed by the standard WebAssembly binary.

### Structure

| Offset | Size | Type | Value | Description |
| :--- | :--- | :--- | :--- | :--- |
| 0 | 4 bytes | ASCII | `WAPP` | Magic Number (0x57, 0x41, 0x50, 0x50) |
| 4 | 1 byte | uint8 | `0x01` | Format Version |
| 5 | N bytes | binary | `...` | Valid version 1.0 WebAssembly Module |

### Parsing Logic
1. Read first 4 bytes. If not `WAPP`, Reject.
2. Read 5th byte. If not `0x01`, Reject (or Handle Migration).
3. Read remaining bytes as WASM Source.

## Key Entities

### Pixel Buffer
- **Type**: Linear Memory Region
- **Format**: 32-bit RGBA values. Each pixel is 4 consecutive bytes: Red, Green, Blue, Alpha (R-G-B-A byte order at sequential memory addresses).
- **Example**: Pure red pixel = bytes `[0xFF, 0x00, 0x00, 0xFF]`
- **Constraints**: Contiguous, size = width × height × 4 bytes.

### Host Window
- **State**:
  - `width`: i32 (Client Area)
  - `height`: i32 (Client Area)
- **Behavior**: Window dimensions snap to Guest-requested size on `update_frame`. User can manually resize window, triggering `on_resize` callback to Guest.

### Input Events
- **Pointer**: Mouse/Touch coordinates relative to Client Area (0,0 is top-left).
- **Buttons**: Mouse Button Indices (Left=1, Middle=2, Right=3).
- **Keys**: SDL Scancodes (Physical Layout).
