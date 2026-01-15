# Research: JSON Header Format Refactor

**Status**: Complete
**Date**: 2026-01-15

## Decisions

### 1. JSON Parsing Library

**Decision**: Use `serde` and `serde_json`.

**Rationale**:
- `serde` is the de-facto standard for serialization in Rust.
- It provides robust, safe, and efficient JSON parsing.
- It allows mapping JSON directly to Rust structs (`WappMetadata`) while confusingly supporting loose parsing (`serde_json::Value`) if needed (though we will use struct mapping for known fields).

**Alternatives Considered**:
- *Manual parsing / lightweight parsers (e.g., `json`, `tinyjson`)*: Rejected due to lack of type safety and robustness compared to `serde`.
- *No dependency (regex/string matching)*: Rejected as unreliable for structured data like JSON (nested objects, escaping, etc.).

### 2. Binary Integer Parsing

**Decision**: Use Rust standard library `u32::from_le_bytes`.

**Rationale**:
- No additional dependency needed (avoids `byteorder` crate).
- Simple and explicit.
- The format strictly specifies Little Endian.

### 3. Window Title Updates

**Decision**: Initialize window with title from metadata.

**Rationale**:
- `sdl2::video::WindowBuilder` accepts a title string during construction.
- `loader.rs` runs before `Graphics::new`, so the title is available when the window is created.
- No dynamic title update needed for this feature (window title is static for the session).

## Implementation Details

### Dependency Changes
`host/Cargo.toml`:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Parsing Logic
1. Read 4 bytes (Magic). Assert "WAPP".
2. Read 4 bytes (Version). `u32::from_le_bytes`. Assert 1.
3. Read 4 bytes (Length). `u32::from_le_bytes`. Let `N = Length`.
4. Read `N` bytes.
5. Parse `N` bytes as UTF-8 string -> `serde_json::from_str` -> `WappMetadata`.
6. Read remaining bytes as WASM.
