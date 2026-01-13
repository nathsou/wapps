# Quickstart Guide: WAPP Header Metadata

**Audience**: Developers implementing or testing the WAPP metadata feature  
**Time to Complete**: 10-15 minutes  
**Prerequisites**: Rust toolchain, repository cloned

## Overview

This feature adds app name and description metadata to the WAPP binary header format. The app name is displayed in the window title bar, and the description is stored for future launcher/file browser use.

**What You'll Do**:
1. Understand the updated binary format
2. Modify loader.rs to parse metadata
3. Update main.rs to display the app name
4. Create a test WAPP file with metadata
5. Verify the implementation manually

## Step 1: Understand the Binary Format

The WAPP file format is now:

```
[4B: "WAPP"] [1B: 0x01] [UTF-8+null: name] [UTF-8+null: desc] [WASM binary]
```

**Example with "My Game" and "A fun game"**:
```
57 41 50 50 01                        // WAPP + version
4D 79 20 47 61 6D 65 00               // "My Game" + null
41 20 66 75 6E 20 67 61 6D 65 00      // "A fun game" + null
00 61 73 6D 01 00 00 00 ...           // WASM binary
```

**Key Points**:
- App name: 1-256 bytes (including null terminator)
- Description: 1-1024 bytes (including null terminator)
- Both are UTF-8 encoded, null-terminated strings
- Empty strings are valid (just `0x00`)

## Step 2: Modify host/src/loader.rs

### 2.1 Add Metadata Struct

Add near the top of the file (after imports):

```rust
/// Metadata parsed from WAPP header
pub struct WappMetadata {
    pub name: String,
    pub description: String,
}
```

### 2.2 Add Helper Function for Null-Terminated Strings

Add this function to read null-terminated UTF-8 strings:

```rust
/// Read a null-terminated UTF-8 string from a byte slice.
///
/// Returns the string and the offset of the byte after the null terminator.
fn read_null_terminated(
    data: &[u8],
    offset: usize,
    max_bytes: usize,
    field_name: &str,
) -> Result<(String, usize)> {
    // Find null terminator within limit
    let end = offset + max_bytes;
    let search_slice = &data[offset..end.min(data.len())];
    
    let null_pos = search_slice
        .iter()
        .position(|&b| b == 0)
        .with_context(|| {
            format!("{} exceeds maximum length of {} bytes", field_name, max_bytes - 1)
        })?;
    
    // Extract bytes before null terminator
    let string_bytes = &search_slice[..null_pos];
    
    // Validate UTF-8
    let string = std::str::from_utf8(string_bytes)
        .with_context(|| format!("Invalid UTF-8 encoding in {}", field_name))?
        .to_string();
    
    // Return string and next offset (after null terminator)
    Ok((string, offset + null_pos + 1))
}
```

### 2.3 Modify load_wapp Function

Change the function signature to return metadata:

```rust
pub fn load_wapp(path: &Path) -> Result<(Vec<u8>, WappMetadata)> {
```

After validating the version byte, add metadata parsing:

```rust
// Parse app name (max 256 bytes including null)
let (name, offset) = read_null_terminated(&data, 5, 256, "App name")?;
debug!("Parsed app name: {:?}", name);

// Parse app description (max 1024 bytes including null)
let (description, offset) = read_null_terminated(&data, offset, 1024, "App description")?;
debug!("Parsed app description: {:?}", description);

// Extract WASM binary (rest of file after metadata)
let wasm_bytes = data[offset..].to_vec();
debug!("WASM module size: {} bytes", wasm_bytes.len());

let metadata = WappMetadata { name, description };

Ok((wasm_bytes, metadata))
```

## Step 3: Update host/src/main.rs

### 3.1 Handle Metadata in Main

Find the `load_wapp` call and update it:

```rust
// Load WAPP file (was: let wasm_bytes = ...)
let (wasm_bytes, metadata) = loader::load_wapp(&wapp_path)?;
```

### 3.2 Set Window Title

After creating the SDL window, set the title:

```rust
// Determine window title
let window_title = if metadata.name.is_empty() {
    // Fallback to filename without extension
    wapp_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("WAPP")
        .to_string()
} else {
    metadata.name.clone()
};

canvas.window_mut().set_title(&window_title)?;
```

## Step 4: Create Test WAPP File

### 4.1 Create Binary Header Script

Create `examples/demo/create_header.sh`:

```bash
#!/bin/bash
set -e

# Usage: ./create_header.sh "App Name" "Description" input.wasm output.wapp

APP_NAME="$1"
APP_DESC="$2"
WASM_FILE="$3"
OUTPUT="$4"

# Write magic + version
printf 'WAPP\x01' > "$OUTPUT"

# Write app name + null terminator
printf "%s\0" "$APP_NAME" >> "$OUTPUT"

# Write description + null terminator
printf "%s\0" "$APP_DESC" >> "$OUTPUT"

# Append WASM binary
cat "$WASM_FILE" >> "$OUTPUT"

echo "Created $OUTPUT with metadata:"
echo "  Name: $APP_NAME"
echo "  Desc: $APP_DESC"
```

Make it executable:
```bash
chmod +x examples/demo/create_header.sh
```

### 4.2 Update Demo Package Script

Modify `examples/demo/package.sh` to use the new header:

```bash
#!/bin/bash
set -e

echo "Building demo WASM module..."
cargo build --release --target wasm32-wasip1

WASM_FILE="target/wasm32-wasip1/release/demo.wasm"
OUTPUT="../../demo.wapp"

# Create WAPP with metadata
./create_header.sh "Demo WAPP" "A simple demonstration application" "$WASM_FILE" "$OUTPUT"

echo "Package created: demo.wapp"
```

### 4.3 Build Test File

```bash
cd examples/demo
./package.sh
cd ../..
```

## Step 5: Test the Implementation

### 5.1 Run with New Format

```bash
cargo run --release -p wapps-host -- demo.wapp
```

**Expected Result**:
- Window opens with title "Demo WAPP"
- Application runs normally
- No errors in console

### 5.2 Test Empty Name (Fallback)

Create a test file with empty name:

```bash
cd examples/demo
./create_header.sh "" "" target/wasm32-wasip1/release/demo.wasm /tmp/empty_meta.wapp
cd ../..

cargo run --release -p wapps-host -- /tmp/empty_meta.wapp
```

**Expected Result**:
- Window opens with title "empty_meta" (filename without .wapp)
- Application runs normally

### 5.3 Test UTF-8 Characters

```bash
cd examples/demo
./create_header.sh "My Game 🎮" "A fun game with emoji" target/wasm32-wasip1/release/demo.wasm /tmp/utf8_meta.wapp
cd ../..

cargo run --release -p wapps-host -- /tmp/utf8_meta.wapp
```

**Expected Result**:
- Window opens with title "My Game 🎮"
- Emoji displays correctly (platform-dependent)

### 5.4 Test Error Cases

**Missing name terminator**:
```bash
# Create invalid file (256 bytes without null)
printf 'WAPP\x01' > /tmp/bad.wapp
for i in {1..256}; do printf 'A' >> /tmp/bad.wapp; done

cargo run --release -p wapps-host -- /tmp/bad.wapp
```

**Expected Result**:
- Error: "App name exceeds maximum length of 255 bytes"
- Program exits gracefully

**Invalid UTF-8**:
```bash
printf 'WAPP\x01\xFF\xFE\x00\x00' > /tmp/bad_utf8.wapp
cat demo.wapp | tail -c +100 >> /tmp/bad_utf8.wapp

cargo run --release -p wapps-host -- /tmp/bad_utf8.wapp
```

**Expected Result**:
- Error: "Invalid UTF-8 encoding in app name"
- Program exits gracefully

## Step 6: Verify Code Quality

### 6.1 Run Formatter

```bash
cargo fmt
```

### 6.2 Run Clippy

```bash
cargo clippy --all-targets
```

**Expected Result**: No warnings

## Common Issues

### Issue: "App name exceeds maximum length"

**Cause**: Null terminator not found within 256 bytes  
**Solution**: Ensure the string includes `\0` and is ≤ 255 bytes of content

### Issue: "Invalid UTF-8 encoding"

**Cause**: Non-UTF-8 bytes in string fields  
**Solution**: Ensure all string content is valid UTF-8

### Issue: Window title shows garbled text

**Cause**: UTF-8 handling issue or font missing glyphs  
**Solution**: 
- Verify UTF-8 encoding in source data
- Check system fonts support the characters

## Next Steps

After completing this quickstart:

1. **Test with your own WAPP files**: Use the `create_header.sh` script
2. **Experiment with metadata**: Try various names, descriptions, special characters
3. **Check error handling**: Verify all validation cases work correctly
4. **Review the code**: Understand the parsing flow in loader.rs

## Reference

- **Specification**: [spec.md](spec.md)
- **Data Model**: [data-model.md](data-model.md)
- **Binary Format Contract**: [contracts/binary-format.md](contracts/binary-format.md)
- **Research Notes**: [research.md](research.md)

## Support

If you encounter issues:
1. Check that demo.wapp was regenerated with the new format
2. Verify Rust version is stable (1.70+)
3. Review error messages for specific validation failures
4. Examine logs with `RUST_LOG=debug` for detailed parsing info
