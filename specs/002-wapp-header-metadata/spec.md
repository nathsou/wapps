# Feature Specification: WAPP Header Metadata Fields

**Feature Branch**: `002-wapp-header-metadata`  
**Created**: 2026-01-13  
**Status**: Draft  
**Input**: User description: "Update the header format to include the app's name, which on desktop is displayed in the window bar, and a short description. Both strings are encoded as UTF-8, null terminated strings."

## Clarifications
### Session 2026-01-13
- Q: How are the metadata fields separated from the WASM binary? → A: Single null byte terminates each string; WASM binary starts immediately after description's null terminator
- Q: What should happen if a string field exceeds its maximum length before finding a null terminator? → A: Reject file with clear error message indicating which field exceeded limits
- Q: What default window title should be displayed when the app name field is empty? → A: Use filename without extension
- Q: How should very long app names be displayed when they exceed the window title bar width? → A: Truncate with ellipsis at the end
- Q: What character encoding should be used for metadata fields? → A: UTF-8 to support international characters

## User Scenarios & Verification *(mandatory)*

### User Story 1 - Display App Name in Window Title (Priority: P1)

As an End User, I want to see the application name in the window title bar so that I can identify which app is running when I have multiple windows open.

**Why this priority**: Core usability improvement - users need to identify apps at a glance.

**Independent Verification**: Can be verified by creating a WAPP file with an app name in the header and confirming the window title displays that name.

**Acceptance Scenarios**:

1. **Given** a valid WAPP file with an app name "My Game" in the header, **When** I run the application, **Then** the desktop window title bar displays "My Game".
2. **Given** a WAPP file with an app name "Particle Simulator", **When** the application is running, **Then** the window title shows "Particle Simulator" instead of the default "WAPP" or file name.

---

### User Story 2 - Include App Description for Metadata (Priority: P2)

As a Developer, I want to embed a short description of my app in the WAPP file so that launchers, app stores, or file browsers can display information about what my app does.

**Why this priority**: Improves discoverability and user understanding of apps without running them.

**Independent Verification**: Can be verified by creating a WAPP file with a description and reading the header metadata programmatically or through a file inspector tool.

**Acceptance Scenarios**:

1. **Given** a WAPP file with description "A simple drawing application", **When** a file inspector reads the header, **Then** the description is correctly extracted as "A simple drawing application".
2. **Given** a WAPP file with description "Real-time physics sandbox", **When** a future launcher tool reads the metadata, **Then** it can display "Real-time physics sandbox" to help users choose which app to launch.

---

### User Story 3 - Handle Missing or Empty Metadata (Priority: P2)

As an End User, I want the system to gracefully handle apps without metadata so that older WAPP files or those without names still work correctly.

**Why this priority**: Backward compatibility and resilience - prevents breaking existing apps.

**Independent Verification**: Can be verified by testing with WAPP files containing empty strings, missing metadata, or legacy version 0x01 headers.

**Acceptance Scenarios**:

1. **Given** a WAPP file named "mygame.wapp" with an empty app name (null terminator only), **When** the application runs, **Then** the window title displays "mygame" (filename without extension).
1. **Given** a WAPP file with minimal metadata (both fields contain only null terminators), **When** the application runs, **Then** the system runs the app successfully with default display values.

---

### Edge Cases

- **Very Long App Name**: System MUST handle app names up to 255 bytes (UTF-8 encoded). Names longer than the window title bar can display should be truncated with ellipsis at the end.
- **Very Long Description**: System MUST handle descriptions up to 1023 bytes (UTF-8 encoded). Extremely long descriptions should be truncated when displayed in UI elements with space constraints.
- **Invalid UTF-8 Sequences**: System MUST reject WAPP files containing invalid UTF-8 byte sequences in name or description fields with error message "Invalid UTF-8 encoding in metadata".
- **Missing Null Terminator**: System MUST reject WAPP files where name or description fields are not properly null-terminated within their maximum byte limits (256 bytes for name, 1024 bytes for description).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST extend the WAPP header format to include two new metadata fields immediately following the version byte: App Name and App Description.
- **FR-002**: App Name field MUST be encoded as a UTF-8 string terminated by a null byte (0x00). Maximum length including null terminator is 256 bytes.
- **FR-003**: App Description field MUST be encoded as a UTF-8 string terminated by a null byte (0x00). Maximum length including null terminator is 1024 bytes.
- **FR-004**: The updated header structure MUST be: Magic Number "WAPP" (4 bytes) + Version byte 0x01 (1 byte) + App Name (null-terminated UTF-8) + App Description (null-terminated UTF-8) + WASM binary.
- **FR-005**: System MUST parse the App Name from the header and display it in the desktop window title bar.
- **FR-006**: System MUST provide a fallback mechanism: if App Name is empty (starts with null terminator), display the filename without extension as the window title.
- **FR-007**: System MUST parse the App Description from the header and make it available for future use by launchers, file inspectors, or app stores.
- **FR-008**: System MUST validate that both App Name and App Description fields contain valid UTF-8 encoded text.
- **FR-009**: System MUST validate that both App Name and App Description fields are properly null-terminated within their maximum byte limits (256 bytes for name, 1024 bytes for description).
- **FR-010**: System MUST reject files that fail header validation (invalid UTF-8 sequences, missing null terminators, exceeding size limits) with a clear error message indicating the specific validation failure.
- **FR-011**: System MUST truncate app names with ellipsis when they exceed the available window title bar display width.

### Key Entities

- **App Name**: A short human-readable UTF-8 string identifying the application (displayed in window title).
- **App Description**: A longer UTF-8 text describing the application's purpose or functionality (for launchers/metadata).
- **WAPP Header**: The complete header structure containing magic number, version byte (0x01), metadata fields (UTF-8 encoded), followed by the WASM binary.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can identify running WAPP applications by their custom names displayed in the window title bar instead of generic labels.
- **SC-002**: Developers can embed app metadata in WAPP files that is correctly parsed and available to the runtime, with 100% of properly formatted headers successfully validated.
- **SC-003**: Header parsing correctly handles all valid UTF-8 characters in app names and descriptions, with clear error messages for invalid UTF-8 sequences or malformed headers.
- **SC-004**: App name is displayed in the window title within 100ms of application startup, providing immediate visual identification.

## Assumptions

- App names will typically be short (under 50 characters) but the system allows up to 255 bytes (UTF-8 encoded) for flexibility.
- Descriptions will be concise summaries (typically 1-2 sentences) but can extend up to 1023 bytes (UTF-8 encoded) for detailed information.
- UTF-8 encoding provides sufficient international character support for the initial version.
- Window title display behavior follows platform conventions (e.g., macOS, Windows, Linux).
- The WASM binary immediately follows the description's null terminator with no padding or alignment requirements.
