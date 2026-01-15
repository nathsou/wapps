# Feature Specification: JSON Header Format Refactor

**Feature Branch**: `003-json-header-format`  
**Created**: 2026-01-15  
**Status**: Draft  
**Input**: User description: "Refactor the hader binary format to store arbitrary metadata: Magic Bytes, Version, Header Length, Header Data (JSON), Binary Blob. It is valid for the JSON object to be empty, no key is required but if present, the 'name' property is used as the window title in the Desktop target."

## User Scenarios & Verification *(mandatory)*

### User Story 1 - Run WAPP with JSON Header (Priority: P1)

As a Developer, I want the WAPP runtime to support a structured binary header containing JSON metadata so that I can bundle arbitrary configuration with my application.

**Why this priority**: this defines the fundamental file format required for all other functionality.

**Independent Verification**: Can be verified by creating a file with the new binary structure and confirming the runtime accepts it and executes the enclosed WASM code.

**Acceptance Scenarios**:

1. **Given** a WAPP file with the new header structure (Magic, Version, Length, JSON, WASM), **When** I run the `wapps-host` with this file, **Then** the WASM application starts successfully.
2. **Given** a WAPP file with an empty JSON object `{}` in the header, **When** I run it, **Then** it executes normally.
3. **Given** a WAPP file with invalid JSON in the header data section, **When** I run it, **Then** the runtime reports a specific error about header corruption and aborts.

---

### User Story 2 - Set Window Title from Metadata (Priority: P2)

As an End User, I want the application window to show the correct application name defined in the package metadata, rather than a generic filename.

**Why this priority**: Improves user experience and application identity.

**Independent Verification**: Create a WAPP file with `{"name": "Super Calc"}` in the header and verify the OS window title.

**Acceptance Scenarios**:

1. **Given** a WAPP file with `{"name": "Super Calc"}` in the JSON header, **When** I launch the app on Desktop, **Then** the window title bar reads "Super Calc".
2. **Given** a WAPP file where the "name" key is missing from the JSON, **When** I launch the app, **Then** the window title defaults to the filename (or "WAPP").
3. **Given** a WAPP file where "name" is an empty string, **When** I launch the app, **Then** the window title uses the default fallback.

---

### Edge Cases

- **Zero-length header**: If Header Length is 0, execution must fail or assume empty JSON? (The spec requires valid JSON, so `{}` is length 2. Length 0 implies empty string which is invalid JSON object unless we allow primitives, but requirement says "JSON object" in user description "It is valid for the JSON object to be empty"). I will stipulate that it must be a valid JSON value, typically an object.
- **Large header**: What is the maximum size of the JSON header? (Implicitly 4GB due to u32, but practical limits should apply).
- **Invalid UTF-8**: If the header data is not valid UTF-8, parsing should fail.
- **Extra bytes**: Does the Version field affect parsing? (Yes, future versions might change layout).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The file format MUST start with the 4-byte Magic Sequence `WAPP`.
- **FR-002**: The Magic Sequence MUST be followed by a 4-byte unsigned integer representing the Format Version (Little Endian).
- **FR-003**: The Version MUST be set to `1` for this initial implementation.
- **FR-004**: The Version MUST be followed by a 4-byte unsigned integer representing the Metadata Header Length (N) in bytes (Little Endian).
- **FR-005**: The Length MUST be followed by exactly N bytes of UTF-8 encoded, valid JSON data.
- **FR-006**: The remainder of the file following the JSON data MUST be treated as the binary WASM module.
- **FR-007**: The Runtime MUST parse the JSON metadata before initializing the WASM module.
- **FR-008**: The Runtime MUST look for a top-level key "name" (string) in the metadata JSON.
- **FR-009**: If the "name" key exists and is a non-empty string, the Runtime MUST use it as the main window title.
- **FR-010**: If the JSON data is malformed or invalid UTF-8, the Runtime MUST terminate with an error.

### Key Entities

- **WAPP File**: The binary container format.
- **Header Metadata**: The JSON object containing "name" and potential future fields.
