#!/usr/bin/env node
import { execSync } from 'node:child_process';
import * as fs from 'node:fs';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Constants
const MAGIC_BYTES = Buffer.from('WAPP');
const VERSION = 1;

// Default Metadata
const DEFAULT_NAME = "Game of Life";
const DEFAULT_DESC = "A simple WAPP for demonstration purposes.";

// Main execution
function main() {
    // 1. Setup paths
    const outputName = process.argv[2] || 'game_of_life.wapp';
    const scriptDir = __dirname;
    const projectRoot = path.resolve(scriptDir, '../..');
    const wasmPath = path.join(scriptDir, 'target/wasm32-wasip1/release/game_of_life.wasm');
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
        const metadata = {
            name: DEFAULT_NAME,
            description: DEFAULT_DESC
        };
        const metadataJson = JSON.stringify(metadata);
        const metadataBuf = Buffer.from(metadataJson, 'utf8');
        const headerLen = metadataBuf.length;

        const wasmContent = fs.readFileSync(wasmPath);
        
        const outputFd = fs.openSync(outputPath, 'w');
        
        // Write Magic (4 bytes)
        fs.writeSync(outputFd, MAGIC_BYTES);
        
        // Write Version (4 bytes, u32 LE)
        const versionBuf = Buffer.alloc(4);
        versionBuf.writeUInt32LE(VERSION, 0);
        fs.writeSync(outputFd, versionBuf);
        
        // Write Header Length (4 bytes, u32 LE)
        const lenBuf = Buffer.alloc(4);
        lenBuf.writeUInt32LE(headerLen, 0);
        fs.writeSync(outputFd, lenBuf);
        
        // Write JSON Metadata
        fs.writeSync(outputFd, metadataBuf);
        
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
