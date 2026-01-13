//! WAPP File Loader
//!
//! Handles parsing and validation of the WAPP binary format:
//! - Bytes 0-3: Magic number "WAPP" (0x57, 0x41, 0x50, 0x50)
//! - Byte 4: Format version (0x01)
//! - Bytes 5+: WebAssembly module binary

use anyhow::{bail, Context, Result};
use log::debug;
use std::fs;
use std::path::Path;

/// Magic bytes for WAPP format
const WAPP_MAGIC: &[u8; 4] = b"WAPP";

/// Current supported format version
const WAPP_VERSION: u8 = 0x01;

/// Minimum valid WAPP file size (header + minimal WASM)
const WAPP_MIN_SIZE: usize = 5 + 8; // 5 byte header + minimal WASM header

/// Load and validate a WAPP file, returning the WASM binary contents.
///
/// # Arguments
/// * `path` - Path to the .wapp file
///
/// # Returns
/// The raw WebAssembly module bytes (without the WAPP header)
///
/// # Errors
/// - File not found or cannot be read
/// - Invalid magic number (not a WAPP file)
/// - Unsupported format version
/// - File too small to contain valid WASM
pub fn load_wapp(path: &Path) -> Result<Vec<u8>> {
    // Read the entire file
    let data =
        fs::read(path).with_context(|| format!("Could not read file: {}", path.display()))?;

    debug!("Read {} bytes from {:?}", data.len(), path);

    // Validate minimum size
    if data.len() < WAPP_MIN_SIZE {
        bail!(
            "Invalid WAPP file: too small ({} bytes). \
            A valid WAPP file must be at least {} bytes.",
            data.len(),
            WAPP_MIN_SIZE
        );
    }

    // Validate magic number
    let magic = &data[0..4];
    if magic != WAPP_MAGIC {
        bail!(
            "Invalid WAPP file: incorrect magic number. \
            Expected 'WAPP' (0x{:02X}{:02X}{:02X}{:02X}), \
            got 0x{:02X}{:02X}{:02X}{:02X}. \
            This file may not be a valid WAPP package.",
            WAPP_MAGIC[0],
            WAPP_MAGIC[1],
            WAPP_MAGIC[2],
            WAPP_MAGIC[3],
            magic[0],
            magic[1],
            magic[2],
            magic[3]
        );
    }

    // Validate version
    let version = data[4];
    if version != WAPP_VERSION {
        bail!(
            "Unsupported WAPP version: {}. \
            This host supports version {} only. \
            The WAPP file may have been created with a newer tool version.",
            version,
            WAPP_VERSION
        );
    }

    debug!("WAPP header valid: magic=WAPP, version={}", version);

    // Extract and return WASM bytes (everything after the 5-byte header)
    let wasm_bytes = data[5..].to_vec();

    // Basic WASM validation: check for WASM magic number
    if wasm_bytes.len() >= 4 {
        let wasm_magic = &wasm_bytes[0..4];
        if wasm_magic != b"\0asm" {
            bail!(
                "Invalid WASM module: incorrect magic number. \
                Expected '\\0asm', got {:?}. \
                The WAPP file may be corrupted.",
                wasm_magic
            );
        }
    }

    Ok(wasm_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wapp_magic_constant() {
        assert_eq!(WAPP_MAGIC, b"WAPP");
    }

    #[test]
    fn test_wapp_version_constant() {
        assert_eq!(WAPP_VERSION, 0x01);
    }
}
