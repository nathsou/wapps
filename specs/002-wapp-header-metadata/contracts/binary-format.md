# WAPP Binary Format Contract v0.01

**Purpose**: Define the binary file format contract for WAPP (WebAssembly Application Package) files with metadata support.

**Version**: 0x01 (with metadata fields)  
**Date**: 2026-01-13

## File Format Specification

### Binary Layout

```
Offset | Size       | Field           | Type           | Required
-------|------------|-----------------|----------------|----------
0      | 4 bytes    | magic           | bytes          | yes
4      | 1 byte     | version         | u8             | yes
5      | 1-256 B    | app_name        | UTF-8+null     | yes
5+N    | 1-1024 B   | app_description | UTF-8+null     | yes
5+N+M  | remaining  | wasm_module     | bytes          | yes

where:
  N = byte length of app_name including null terminator (1 ≤ N ≤ 256)
  M = byte length of app_description including null terminator (1 ≤ M ≤ 1024)
```

## Field Contracts

### magic (4 bytes)

**Contract**: Fixed byte sequence identifying WAPP format

**Value**: `0x57 0x41 0x50 0x50` ("WAPP" in ASCII)

**Validation**:
```
MUST be exactly these 4 bytes in this order
MUST NOT be any other value
```

**Violations**:
- File does not start with 0x57414050 → REJECT "Invalid WAPP file: incorrect magic number"

---

### version (1 byte)

**Contract**: Format version indicator

**Value**: `0x01`

**Validation**:
```
MUST equal 0x01
MUST NOT be 0x00 or 0x02-0xFF
```

**Violations**:
- version ≠ 0x01 → REJECT "Unsupported WAPP version: {actual_version}"

---

### app_name (1-256 bytes)

**Contract**: UTF-8 encoded application name, null-terminated

**Format**:
```
<UTF-8 bytes> 0x00
```

**Requirements**:
```
MUST contain at least one byte (the null terminator)
MUST contain at most 256 bytes total (including null terminator)
MUST have exactly one null terminator (0x00)
MUST form valid UTF-8 when null terminator excluded
MAY be empty string (single 0x00 byte)
```

**Constraints**:
- Minimum: 1 byte (empty string: `0x00`)
- Maximum: 256 bytes (255 bytes content + `0x00`)
- Character set: Any valid UTF-8 codepoint
- Multi-byte sequences: Allowed (emoji, CJK, etc.)

**Validation**:
```
1. Scan forward from offset 5 for null byte (0x00)
2. IF null not found within 256 bytes → REJECT "App name exceeds maximum length"
3. Extract bytes before null as potential UTF-8
4. IF bytes are not valid UTF-8 → REJECT "Invalid UTF-8 encoding in app name"
5. ELSE accept as app_name, next offset = position after null
```

**Violations**:
- No null found in 256 bytes → REJECT "Missing null terminator in app name"
- Invalid UTF-8 sequence → REJECT "Invalid UTF-8 encoding in app name"

**Examples**:
```
Valid:
  0x00                              (empty string)
  0x48 0x69 0x00                    ("Hi")
  0xF0 0x9F 0x98 0x80 0x00          (😀 emoji)
  0xE4 0xBD 0xA0 0xE5 0xA5 0xBD 0x00 ("你好" Chinese)

Invalid:
  <256 bytes of non-null data>      (no terminator)
  0xFF 0xFE 0x00                    (invalid UTF-8)
  0xC0 0x80 0x00                    (overlong encoding)
```

---

### app_description (1-1024 bytes)

**Contract**: UTF-8 encoded application description, null-terminated

**Format**:
```
<UTF-8 bytes> 0x00
```

**Requirements**:
```
MUST contain at least one byte (the null terminator)
MUST contain at most 1024 bytes total (including null terminator)
MUST have exactly one null terminator (0x00)
MUST form valid UTF-8 when null terminator excluded
MAY be empty string (single 0x00 byte)
```

**Constraints**:
- Minimum: 1 byte (empty string: `0x00`)
- Maximum: 1024 bytes (1023 bytes content + `0x00`)
- Character set: Any valid UTF-8 codepoint
- Multi-byte sequences: Allowed

**Validation**:
```
1. Scan forward from offset (5+N) for null byte (0x00)
2. IF null not found within 1024 bytes → REJECT "Description exceeds maximum length"
3. Extract bytes before null as potential UTF-8
4. IF bytes are not valid UTF-8 → REJECT "Invalid UTF-8 encoding in description"
5. ELSE accept as app_description, next offset = position after null
```

**Violations**:
- No null found in 1024 bytes → REJECT "Missing null terminator in description"
- Invalid UTF-8 sequence → REJECT "Invalid UTF-8 encoding in description"

---

### wasm_module (remaining bytes)

**Contract**: WebAssembly module binary

**Format**: Standard WebAssembly binary format (.wasm)

**Requirements**:
```
MUST start immediately after app_description null terminator
MUST be valid WebAssembly module (validated by runtime)
MUST NOT have padding or alignment bytes before it
```

**Constraints**:
- Minimum size: 8 bytes (WASM magic + version)
- Maximum size: Unbounded (limited by file system/memory)

**Validation**:
```
1. Extract remaining bytes after app_description
2. Pass to WebAssembly runtime for validation
3. Runtime handles WASM-specific validation
```

## Implementation Contracts

### Parser Contract

**Requirements for WAPP Parser Implementations**:

```
MUST validate magic number before continuing
MUST validate version before continuing
MUST validate each string field is null-terminated within limits
MUST validate each string field contains valid UTF-8
MUST reject files immediately on first validation failure
MUST provide clear error messages indicating which field failed
MUST NOT modify or "fix" invalid files
MUST extract WASM module starting at byte after description's null
SHOULD fail fast (don't continue parsing after error)
```

### Runtime Contract

**Requirements for WAPP Runtime Implementations**:

```
MUST display app_name in window title (if not empty)
MUST use filename without extension as fallback if app_name is empty
MUST make app_description available for future use
MUST handle UTF-8 display correctly (no mojibake)
MAY truncate long app_name with ellipsis for display
SHOULD log validation errors for debugging
```

## Versioning Contract

**Version Stability**:
```
Version 0x01 format is STABLE
Parsers MUST reject versions ≠ 0x01
Future versions (0x02+) MAY have different layouts
Version byte determines format interpretation
```

**Backward Compatibility**:
```
Version 0x01 has no backward compatibility with earlier formats
All 0x01 files MUST include metadata fields
```

## Security Contract

**Security Requirements**:

```
MUST bound all string reads (prevent unbounded reads)
MUST validate UTF-8 (prevent invalid/malicious sequences)
MUST NOT execute WASM before validation complete
MUST reject files with validation errors (no partial loading)
MUST NOT trust file size claims (read actual bytes)
```

**Attack Vectors to Prevent**:
- Buffer overrun: Bounded by 256/1024 byte limits
- Invalid UTF-8: Rejected by validation
- Missing terminators: Rejected when limit reached
- Malicious WASM: Handled by wasmtime sandbox

## Test Scenarios

### Valid Files

```
1. Empty metadata:
   Magic + Version + 0x00 + 0x00 + WASM
   
2. ASCII name, empty description:
   Magic + Version + "Hello" + 0x00 + 0x00 + WASM
   
3. Full UTF-8 with emoji:
   Magic + Version + "My Game 🎮" + 0x00 + "A fun game" + 0x00 + WASM
   
4. Maximum length fields:
   Magic + Version + <255 bytes> + 0x00 + <1023 bytes> + 0x00 + WASM
```

### Invalid Files

```
1. Wrong magic:
   0xFF 0xFF 0xFF 0xFF + ... → REJECT
   
2. Wrong version:
   Magic + 0x02 + ... → REJECT
   
3. Missing name terminator:
   Magic + Version + <256 bytes without null> → REJECT
   
4. Invalid UTF-8 in name:
   Magic + Version + 0xFF 0xFE + 0x00 → REJECT
   
5. Missing description terminator:
   Magic + Version + "Name" + 0x00 + <1024 bytes without null> → REJECT
   
6. Invalid UTF-8 in description:
   Magic + Version + "Name" + 0x00 + 0xC0 0x80 + 0x00 → REJECT
```

## Reference Implementation Checklist

- [ ] Parse magic number (4 bytes at offset 0)
- [ ] Parse version byte (1 byte at offset 4)
- [ ] Parse app_name (null-terminated, max 256 bytes)
  - [ ] Find null terminator within limit
  - [ ] Validate UTF-8
  - [ ] Store as String
- [ ] Parse app_description (null-terminated, max 1024 bytes)
  - [ ] Find null terminator within limit
  - [ ] Validate UTF-8
  - [ ] Store as String
- [ ] Extract WASM module (remaining bytes)
- [ ] Display app_name in window title
- [ ] Provide fallback to filename if name empty
- [ ] Return clear errors on validation failure
