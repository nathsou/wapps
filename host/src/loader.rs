//! WAPP File Loader
//!
//! Handles parsing and validation of the WAPP binary format:
//! - Bytes 0-3: Magic number "WAPP" (0x57, 0x41, 0x50, 0x50)
//! - Bytes 4-7: Format version (1, u32 LE)
//! - Bytes 8-11: Header Length (N, u32 LE)
//! - Bytes 12..12+N: JSON Metadata (UTF-8)
//! - Bytes 12+N+: WebAssembly module binary

use anyhow::{bail, Context, Result};
use log::debug;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Magic bytes for WAPP format
const WAPP_MAGIC: &[u8; 4] = b"WAPP";

/// Current supported format version
const WAPP_VERSION: u32 = 1;

/// Minimum valid WAPP file size (4 magic + 4 version + 4 length + 2 json {})
const WAPP_MIN_SIZE: usize = 4 + 4 + 4 + 2;

/// Metadata parsed from the WAPP header
#[derive(Debug, Clone, Default, Deserialize)]
pub struct WappMetadata {
    /// Application name
    #[serde(default)]
    pub name: String,
    /// Application description
    #[serde(default)]
    pub description: String,
}

/// Load and validate a WAPP file, returning the WASM binary contents.
///
/// # Arguments
/// * `path` - Path to the .wapp file
///
/// # Returns
/// The raw WebAssembly module bytes (without the WAPP header) and metadata
///
/// # Errors
/// - File not found or cannot be read
/// - Invalid magic number (not a WAPP file)
/// - Unsupported format version
/// - File too small to contain valid WASM
/// - Invalid metadata (invalid JSON)
pub fn load_wapp(path: &Path) -> Result<(Vec<u8>, WappMetadata)> {
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

    // Validate version (Bytes 4-7, u32 LE)
    let version_bytes: [u8; 4] = data[4..8].try_into().expect("slice with incorrect length");
    let version = u32::from_le_bytes(version_bytes);
    if version != WAPP_VERSION {
        bail!(
            "Unsupported WAPP version: {}. \
            This host supports version {} only. \
            The WAPP file may have been created with a newer tool version.",
            version,
            WAPP_VERSION
        );
    }

    // Parse Header Length (Bytes 8-11, u32 LE)
    let length_bytes: [u8; 4] = data[8..12].try_into().expect("slice with incorrect length");
    let header_len = u32::from_le_bytes(length_bytes) as usize;

    debug!("WAPP header: magic=WAPP, version={}, length={}", version, header_len);

    // Validate total size again with header length
    // 4 magic + 4 version + 4 length + header_len
    let header_end = 12 + header_len;
    if data.len() < header_end {
        bail!(
            "Invalid WAPP file: incomplete header. \
            Expected {} bytes for header metadata, but file ends at byte {}.",
            header_len,
            data.len()
        );
    }

    // Parse JSON Metadata
    let json_bytes = &data[12..header_end];
    let metadata: WappMetadata = serde_json::from_slice(json_bytes)
        .context("Failed to parse WAPP header metadata (invalid JSON)")?;
    
    debug!("Parsed Metadata: name={:?}, description={:?}", metadata.name, metadata.description);

    // Extract and return WASM bytes (everything after the header)
    let wasm_bytes = data[header_end..].to_vec();

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

    Ok((wasm_bytes, metadata))
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
