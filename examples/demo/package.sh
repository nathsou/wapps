#!/bin/bash
# Package script for creating a WAPP file from the demo
# Usage: ./package.sh [output_name]

set -e

OUTPUT_NAME="${1:-demo.wapp}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "Building WASM module..."
cd "$SCRIPT_DIR"
cargo build --target wasm32-wasip1 --release

WASM_FILE="$SCRIPT_DIR/target/wasm32-wasip1/release/demo.wasm"

if [ ! -f "$WASM_FILE" ]; then
    echo "Error: WASM file not found at $WASM_FILE"
    exit 1
fi

OUTPUT_PATH="$PROJECT_ROOT/$OUTPUT_NAME"

echo "Creating WAPP package..."
# Write WAPP header (magic "WAPP" + version 0x01)
printf 'WAPP\x01' > "$OUTPUT_PATH"

# Append WASM binary
cat "$WASM_FILE" >> "$OUTPUT_PATH"

WASM_SIZE=$(stat -f%z "$WASM_FILE" 2>/dev/null || stat -c%s "$WASM_FILE")
WAPP_SIZE=$(stat -f%z "$OUTPUT_PATH" 2>/dev/null || stat -c%s "$OUTPUT_PATH")

echo "Done!"
echo "  WASM size: $WASM_SIZE bytes"
echo "  WAPP size: $WAPP_SIZE bytes"
echo "  Output: $OUTPUT_PATH"
echo ""
echo "Run with: cargo run --release -p wapps-host -- $OUTPUT_NAME"
