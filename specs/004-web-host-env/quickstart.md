# Quickstart: Building & Running the Web Host

## Prerequisites

1. **Rust & WASM Toolchain**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   cargo install wasm-pack
   ```

2. **Python** (for simple HTTP server during development):
   ```bash
   python3 --version
   ```

## Building the Runtime

1. Navigate to the web runtime directory:
   ```bash
   cd host/web-runtime
   ```

2. Build with wasm-pack:
   ```bash
   wasm-pack build --target web
   ```
   This generates the `pkg/` directory containing the JS glue code and `.wasm` binary.

## Running the Launcher

1. Navigate to the launcher directory:
   ```bash
   cd host/web-launcher
   ```

2. Link the runtime (copy/symlink):
   ```bash
   # Copy the pkg build to the launcher's source
   cp -r ../web-runtime/pkg/ ./lib
   ```

3. Serve the directory:
   ```bash
   # Must use a server to support WASM mime types
   python3 -m http.server 8080
   ```

4. Open `http://localhost:8080` in your browser.

## Deploying

The project includes a GitHub Action `web-host-deploy.yml`.
Simply push to `main` (or the feature branch if configured) and check the "Actions" tab. The site will be available at imports `https://<org>.github.io/wapps/`.
