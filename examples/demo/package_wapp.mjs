#!/usr/bin/env node
import { execSync } from 'node:child_process';
import * as fs from 'node:fs';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Constants
const MAGIC_BYTES = Buffer.from('WAPP');
const VERSION_BYTES = Buffer.from([0x01]);
const MAX_NAME_LEN = 256;
const MAX_DESC_LEN = 1024;

// Default Metadata
const DEFAULT_NAME = "Game of Life WAPP";
const DEFAULT_DESC = "A simple WAPP for demonstration purposes.";

// Main execution
function main() {
    // 1. Setup paths
    const outputName = process.argv[2] || 'demo.wapp';
    const scriptDir = __dirname;
    const projectRoot = path.resolve(scriptDir, '../..');
    const wasmPath = path.join(scriptDir, 'target/wasm32-wasip1/release/demo.wasm');
    const outputPath = path.join(projectRoot, outputName);

    // 2. Build WASM
    console.log('Building WASM module...');
    try {
        execSync('cargo build --target wasm32-wasip1 --release', { cwd: scriptDir, stdio: 'inherit' });
    } catch (e) {
        console.error('Build failed.');
        process.exit(1);
    }

    if (!fs.existsSync(wasmPath)) {
        console.error(`Error: WASM file not found at ${wasmPath}`);
        process.exit(1);
    }

    // 3. Create WAPP File (Header + Content)
    console.log('Creating WAPP package...');
    try {
        const name = DEFAULT_NAME;
        const description = DEFAULT_DESC;

        // Create Buffers and Validate
        const nameBuf = Buffer.from(name, 'utf8');
        if (nameBuf.length >= MAX_NAME_LEN) {
            throw new Error(`Name too long (max ${MAX_NAME_LEN - 1} bytes)`);
        }
        
        const descBuf = Buffer.from(description, 'utf8');
        if (descBuf.length >= MAX_DESC_LEN) {
            throw new Error(`Description too long (max ${MAX_DESC_LEN - 1} bytes)`);
        }

        const wasmContent = fs.readFileSync(wasmPath);
        
        const outputFd = fs.openSync(outputPath, 'w');
        
        // Write Header
        fs.writeSync(outputFd, MAGIC_BYTES);
        fs.writeSync(outputFd, VERSION_BYTES);
        
        // Write Name + Null Terminator
        fs.writeSync(outputFd, nameBuf);
        fs.writeSync(outputFd, Buffer.from([0x00])); 
        
        // Write Description + Null Terminator
        fs.writeSync(outputFd, descBuf);
        fs.writeSync(outputFd, Buffer.from([0x00])); 
        
        // Write Body
        fs.writeSync(outputFd, wasmContent);
        
        fs.closeSync(outputFd);

        // 4. Report Success
        const wasmSize = fs.statSync(wasmPath).size;
        const wappSize = fs.statSync(outputPath).size;
        
        console.log('Done!');
        console.log(`  WASM size: ${wasmSize} bytes`);
        console.log(`  WAPP size: ${wappSize} bytes`);
        console.log(`  Output: ${outputPath}`);
        console.log('');
        console.log(`Run with: cargo run --release -p wapps-host -- ${outputName}`);
        
    } catch (e) {
        console.error('Packaging failed:', e.message);
        process.exit(1);
    }
}

main();
