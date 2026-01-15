# Quickstart: Creating a WAPP File with JSON Header

This guide demonstrates how to manually create a valid WAPP v1 file with a JSON header.

## Prerequisites
- A WASM file (e.g., `app.wasm`)
- Python 3 (for script generation)

## Python Script

Save this as `package_wapp.py`:

```python
import struct
import json
import sys

def create_wapp(wasm_path, output_path, metadata):
    # 1. Prepare Metadata
    json_bytes = json.dumps(metadata).encode('utf-8')
    header_len = len(json_bytes)
    
    # 2. Read WASM
    with open(wasm_path, 'rb') as f:
        wasm_bytes = f.read()

    # 3. Construct Header
    # Magic: "WAPP"
    # Version: 1 (u32 le)
    # Length: N (u32 le)
    header = b'WAPP' + \
             struct.pack('<I', 1) + \
             struct.pack('<I', header_len) + \
             json_bytes

    # 4. Write File
    with open(output_path, 'wb') as f:
        f.write(header)
        f.write(wasm_bytes)
    
    print(f"Created {output_path}")
    print(f"Header Length: {header_len}")
    print(f"Metadata: {metadata}")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python package_wapp.py <input.wasm> <output.wapp>")
        sys.exit(1)
        
    metadata = {
        "name": "My WAPP App",
        "description": "Created with Python packager"
    }
    
    create_wapp(sys.argv[1], sys.argv[2], metadata)
```

## Running

```bash
# Create a dummy WASM if you don't have one
echo -ne '\x00\x61\x73\x6d\x01\x00\x00\x00' > empty.wasm

# Package it
python3 package_wapp.py empty.wasm my_app.wapp

# Run it
cargo run -p wapps-host --release -- my_app.wapp
```
