# Data Model: WAPP Header Metadata Fields

**Date**: 2026-01-13  
**Feature**: [spec.md](spec.md)

## Binary Format Structure

### WAPP File Format (Version 0x01 with Metadata)

```
┌─────────────────────────────────────────────────────────────┐
│ WAPP File Structure                                         │
├─────────────────────────────────────────────────────────────┤
│ Offset │ Size      │ Field        │ Description             │
├────────┼───────────┼──────────────┼─────────────────────────┤
│ 0      │ 4 bytes   │ Magic        │ "WAPP" (0x57415050)     │
│ 4      │ 1 byte    │ Version      │ 0x01                    │
│ 5      │ 1-256 B   │ App Name     │ UTF-8, null-terminated  │
│ 5+N    │ 1-1024 B  │ Description  │ UTF-8, null-terminated  │
│ 5+N+M  │ Variable  │ WASM Binary  │ WebAssembly module      │
└─────────────────────────────────────────────────────────────┘

where N = length of app name including null terminator (1-256)
      M = length of description including null terminator (1-1024)
```

### Field Specifications

#### Magic Number (4 bytes, offset 0)
- **Type**: Fixed byte sequence
- **Value**: `[0x57, 0x41, 0x50, 0x50]` ("WAPP" in ASCII)
- **Validation**: Must match exactly
- **Error**: "Invalid WAPP file: incorrect magic number"

#### Version (1 byte, offset 4)
- **Type**: Unsigned 8-bit integer
- **Value**: `0x01`
- **Validation**: Must equal 0x01
- **Error**: "Unsupported WAPP version: {version}"

#### App Name (1-256 bytes, offset 5)
- **Type**: UTF-8 encoded string
- **Termination**: Null byte (0x00)
- **Min Length**: 1 byte (empty string: just null terminator)
- **Max Length**: 256 bytes (including null terminator)
- **Character Set**: Any valid UTF-8 sequence
- **Validation Rules**:
  1. Must find null terminator within 256 bytes from offset 5
  2. Bytes before null must form valid UTF-8
  3. Null terminator itself is not part of the string content
- **Errors**:
  - "App name exceeds maximum length of 255 bytes"
  - "Invalid UTF-8 encoding in app name"
  - "Missing null terminator in app name"

#### App Description (1-1024 bytes, offset 5+N)
- **Type**: UTF-8 encoded string
- **Termination**: Null byte (0x00)
- **Min Length**: 1 byte (empty string: just null terminator)
- **Max Length**: 1024 bytes (including null terminator)
- **Character Set**: Any valid UTF-8 sequence
- **Validation Rules**:
  1. Must find null terminator within 1024 bytes from start
  2. Bytes before null must form valid UTF-8
  3. Null terminator itself is not part of the string content
- **Errors**:
  - "App description exceeds maximum length of 1023 bytes"
  - "Invalid UTF-8 encoding in app description"
  - "Missing null terminator in app description"

#### WASM Binary (variable length, offset 5+N+M)
- **Type**: WebAssembly module binary
- **Length**: Remainder of file
- **Validation**: Handled by wasmtime (not part of header parsing)

## Runtime Data Structures

### WappMetadata

Represents the parsed metadata from a WAPP file header.

```rust
pub struct WappMetadata {
    /// Application name to display in window title.
    /// Empty string if the header contained only a null terminator.
    pub name: String,
    
    /// Application description for launchers/file browsers.
    /// Empty string if the header contained only a null terminator.
    pub description: String,
}
```

**Properties**:
- Both fields are owned `String` types (allocated on heap)
- Empty strings are valid (represent metadata with just null terminator)
- UTF-8 validation happens during parsing, so these are guaranteed valid

**Usage**:
```rust
// Example usage (not implementation)
let metadata = parse_metadata(&file_data)?;
window.set_title(&metadata.name);
```

### WappFile

Represents a complete parsed WAPP file.

```rust
pub struct WappFile {
    /// Parsed metadata from header
    pub metadata: WappMetadata,
    
    /// Raw WASM module bytes (without WAPP header)
    pub wasm_bytes: Vec<u8>,
}
```

**Properties**:
- `wasm_bytes` contains only the WebAssembly module (header stripped)
- Ready to pass directly to wasmtime for execution
- Metadata available for display/inspection

## Parsing State Machine

The parsing process follows a strict sequential flow:

```
┌──────────┐
│  Start   │
└────┬─────┘
     ↓
┌────────────────┐
│ Read 4 bytes   │
│ Validate Magic │ ──── Error: Invalid magic
└────┬───────────┘
     ↓
┌────────────────┐
│ Read 1 byte    │
│ Validate Ver   │ ──── Error: Unsupported version
└────┬───────────┘
     ↓
┌────────────────────┐
│ Read until null or │
│ 256 bytes          │ ──── Error: No null found
│ Parse App Name     │ ──── Error: Invalid UTF-8
└────┬───────────────┘
     ↓
┌────────────────────┐
│ Read until null or │
│ 1024 bytes         │ ──── Error: No null found
│ Parse Description  │ ──── Error: Invalid UTF-8
└────┬───────────────┘
     ↓
┌────────────────┐
│ Extract WASM   │
│ (rest of file) │
└────┬───────────┘
     ↓
┌──────────┐
│ Success  │
└──────────┘
```

**State Tracking**:
- Current byte offset into file buffer
- Each parse step returns next offset
- Early return on any validation failure
- No backtracking needed (sequential parse)

## Constraints & Invariants

### Byte Length Limits
- **App Name**: Maximum 255 bytes of content + 1 null = 256 bytes total
- **App Description**: Maximum 1023 bytes of content + 1 null = 1024 bytes total
- **Total Header**: 5 (magic + version) + 256 + 1024 = 1285 bytes maximum

### UTF-8 Invariants
- All string content must be valid UTF-8
- Null terminator (0x00) is not part of UTF-8 content
- Multi-byte UTF-8 sequences can span up to 4 bytes
- Character count ≠ byte count (emoji may be 4 bytes)

### Format Invariants
- Magic number is always exactly 4 bytes
- Version is always exactly 1 byte
- Every string field has exactly one null terminator
- WASM binary starts immediately after description's null (no padding)

## Display Rules

### Window Title Display

**Primary**: Show parsed app name
```
metadata.name (if not empty)
```

**Fallback**: Show filename without extension
```
Path::file_stem(wapp_file_path)
```

**Truncation**: Platform-specific handling
- If name exceeds window title bar width, truncate with "…"
- Exact width depends on font, OS, window decorations
- Implementation may delegate to SDL2/OS

### Description Display

**Current**: Not displayed (stored for future use)
**Future**: Available for file browsers, launchers, app stores
- Full description can be read via file inspection tools
- May be truncated in UI tooltips or preview panes

## Validation Strategy

### Parse-Time Validation
1. **Magic Number**: Byte-exact comparison
2. **Version**: Exact value match
3. **String Fields**: 
   - Find null terminator within byte limit
   - Validate UTF-8 encoding
   - Extract as String

### Post-Parse Validation
- WASM module validation handled by wasmtime (outside header scope)
- No semantic validation of name/description content
- Empty strings are valid

### Error Recovery
- **No recovery**: Any validation failure rejects entire file
- **Clear errors**: Specific message indicates which field failed
- **No partial loading**: All-or-nothing approach for safety
