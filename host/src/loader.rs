//! WAPP File Loader
//!
//! Handles parsing and validation of the WAPP binary format:
//! - Bytes 0-3: Magic number "WAPP" (0x57, 0x41, 0x50, 0x50)
//! - Byte 4: Format version (0x01)
//! - Bytes 5..N: Application Name (UTF-8, null-terminated, max 256 bytes)
//! - Bytes N+1..M: Application Description (UTF-8, null-terminated, max 1024 bytes)
//! - Bytes M+1+: WebAssembly module binary

use anyhow::{bail, Context, Result};
use log::debug;
use std::fs;
use std::path::Path;

/// Magic bytes for WAPP format
const WAPP_MAGIC: &[u8; 4] = b"WAPP";

/// Current supported format version
const WAPP_VERSION: u8 = 0x01;

/// Minimum valid WAPP file size (header + minimal WASM)
const WAPP_MIN_SIZE: usize = 5 + 1 + 1 + 8; // 5 byte header + 1 name null + 1 desc null + minimal WASM header

/// Metadata parsed from the WAPP header
#[derive(Debug, Clone, Default)]
pub struct WappMetadata {
    /// Application name (UTF-8, null-terminated in binary)
    pub name: String,
    /// Application description (UTF-8, null-terminated in binary)
    pub description: String,
}

/// Helper to read a null-terminated UTF-8 string from a byte slice.
///
/// Returns (content, bytes_consumed)
fn read_null_terminated(data: &[u8], max_len: usize) -> Result<(String, usize)> {
    let limit = max_len.min(data.len());
    if let Some(pos) = data[..limit].iter().position(|&b| b == 0) {
        let s = std::str::from_utf8(&data[..pos])
            .context("Invalid UTF-8 in string field")?
            .to_string();
        Ok((s, pos + 1))
    } else {
        bail!("String field missing null terminator or exceeds maximum length")
    }
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
/// - Invalid metadata (missing null terminators, invalid UTF-8)
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

    // Start parsing metadata
    let mut offset = 5;

    // Parse App Name
    let (name, consumed) =
        read_null_terminated(&data[offset..], 256).context("Failed to parse App Name")?;
    offset += consumed;
    debug!("Parsed App Name: {:?}", name);

    // Parse App Description
    let (description, consumed) =
        read_null_terminated(&data[offset..], 1024).context("Failed to parse App Description")?;
    offset += consumed;
    debug!("Parsed App Description: {:?}", description);

    let metadata = WappMetadata { name, description };

    // Extract and return WASM bytes (everything after the header)
    let wasm_bytes = data[offset..].to_vec();

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
