# Research: WAPP Header Metadata Fields

**Date**: 2026-01-13  
**Feature**: [spec.md](spec.md)

## Research Tasks

All unknowns from Technical Context have been resolved through specification clarifications and existing codebase analysis. No additional research required.

## Decisions & Rationale

### 1. UTF-8 Encoding for Metadata

**Decision**: Use UTF-8 encoding for app name and description fields

**Rationale**:
- Provides international character support (emoji, non-Latin scripts, accented characters)
- Standard encoding with robust validation available in Rust std library
- Variable-length encoding means short ASCII strings remain compact
- Well-defined error conditions for invalid sequences

**Alternatives Considered**:
- ASCII-only: Rejected due to lack of international character support
- UTF-16: Rejected due to complexity and larger minimum size per character
- Latin-1: Rejected due to limited character set

**Implementation Notes**:
- Use `std::str::from_utf8()` for zero-copy validation
- Reject files with invalid UTF-8 sequences at load time
- No transcoding needed - display directly in SDL2 window title

### 2. Null-Terminated Strings with Byte Limits

**Decision**: Use null-terminated strings with maximum byte limits (256 for name, 1024 for description)

**Rationale**:
- Simple, well-understood format from C tradition
- Allows variable-length strings without additional length prefix
- Byte limits prevent unbounded memory reads and parsing time
- Clear error conditions when null terminator not found within limit

**Alternatives Considered**:
- Length-prefixed (Pascal strings): Rejected to maintain simplicity and avoid need for 2-byte or 4-byte length fields
- Fixed-size with padding: Rejected due to wasted space (forces all files to use max space)
- Offset table: Rejected as over-engineered for 2 fields

**Implementation Notes**:
- Sequential parsing: read bytes until null or limit reached
- Validate UTF-8 on the complete string (excluding null terminator)
- WASM binary starts immediately after description's null byte

### 3. Same Version Byte (0x01)

**Decision**: Keep version byte at 0x01 for files with metadata

**Rationale**:
- All new WAPP files will include metadata fields going forward
- Simplifies implementation - no version branching logic needed
- Existing v0.01 files in the wild can be considered legacy/deprecated
- Clear break from old format to new format

**Alternatives Considered**:
- Bump to 0x02: Rejected per user requirement
- Optional fields with markers: Rejected due to added complexity

**Implementation Notes**:
- Loader always expects metadata fields after version byte
- No backward compatibility code needed
- Existing demo.wapp file will need to be regenerated with metadata

### 4. Error Handling Strategy

**Decision**: Strict validation with clear error messages; reject invalid files

**Rationale**:
- Prevents undefined behavior and potential security issues
- Makes format requirements explicit and enforceable
- Helps developers identify issues during development
- Aligns with Rust's safety-first philosophy

**Validation Rules**:
1. App name must be null-terminated within 256 bytes
2. App description must be null-terminated within 1024 bytes
3. Both fields must contain valid UTF-8
4. Exceeding byte limits → reject with "Field exceeds maximum length"
5. Invalid UTF-8 → reject with "Invalid UTF-8 encoding in metadata"
6. Missing null terminator → reject with "Missing null terminator"

**Implementation Notes**:
- Use anyhow's Context for descriptive error messages
- Early return on first validation failure
- Log validation errors for debugging

### 5. Window Title Display

**Decision**: 
- Primary: Display parsed app name in window title
- Fallback: Display filename without extension if name is empty (just null terminator)
- Truncation: Add ellipsis if name exceeds window title bar width

**Rationale**:
- Provides immediate visual identification of apps
- Filename fallback ensures useful default
- Platform-specific truncation handled by SDL2/OS in most cases

**Implementation Notes**:
- Extract filename stem using `Path::file_stem()`
- SDL2 `set_window_title()` accepts UTF-8 strings natively
- Ellipsis truncation can be added in display layer if needed

### 6. No New Dependencies

**Decision**: Use only Rust standard library for UTF-8 validation and string handling

**Rationale**:
- Aligns with Constitution principle IV (Simplicity - minimal dependencies)
- std::str provides all needed functionality
- No external crates required for basic string operations

**Implementation Notes**:
- `std::str::from_utf8()` - validate UTF-8
- `Path::file_stem()` - extract filename without extension  
- No regex or complex parsing libraries needed

## Best Practices

### Binary Format Parsing in Rust

**Pattern**: Sequential buffer reading with Result propagation

```rust
// Example structure (not implementation)
fn parse_metadata(data: &[u8], offset: usize) -> Result<(Metadata, usize)> {
    let (name, next_offset) = read_null_terminated(data, offset, 256)?;
    let (description, next_offset) = read_null_terminated(data, next_offset, 1024)?;
    Ok((Metadata { name, description }, next_offset))
}
```

**Key Practices**:
- Use byte slices for zero-copy parsing
- Return new offset after each field
- Propagate errors with ? operator
- Validate incrementally (fail fast)

### UTF-8 Validation

**Pattern**: from_utf8 with context

```rust
// Example structure (not implementation)
let name_str = std::str::from_utf8(name_bytes)
    .context("Invalid UTF-8 in app name")?;
```

**Key Practices**:
- Validate after extracting null-terminated bytes
- Use from_utf8 (returns Result) not from_utf8_unchecked
- Add context to errors for debugging
- No need for lossy conversion (reject invalid)

## Technology Stack Summary

| Component | Technology | Justification |
|-----------|------------|---------------|
| String Encoding | UTF-8 | International character support, Rust native |
| String Format | Null-terminated | Simple, well-understood, variable-length |
| Validation | std::str::from_utf8 | Zero-copy, robust, no dependencies |
| Error Handling | anyhow::Result | Existing project pattern, context propagation |
| Display | SDL2 window title | Existing dependency, UTF-8 support |

## Open Questions

None - all clarifications resolved through specification process.
