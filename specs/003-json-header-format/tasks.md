# Implementation Tasks: JSON Header Format Refactor

**Branch**: `003-json-header-format` | **Spec**: [specs/003-json-header-format/spec.md](spec.md)

## Dependencies

- **Phase 1: Setup**
  - None
- **Phase 2: US1 (JSON Header Support)**
  - Depends on Phase 1
- **Phase 3: US2 (Window Title)**
  - Depends on Phase 2
- **Phase 4: Demo Update**
  - Depends on Phase 2 (Runtime must support the format before we update the demo to use it, otherwise we break the build in between)
- **Phase 5: Polish**
  - Depends on all above

---

## Phase 1: Setup

- [x] T001 Add `serde` and `serde_json` dependencies to `host/Cargo.toml`

## Phase 2: User Story 1 - Run WAPP with JSON Header

**Goal**: The runtime accepts and parses the new binary format with JSON metadata.
**Independent Verification**: Unit test `load_wapp` or run a manually created WAPP file with the new format (as per Quickstart).

- [x] T002 [US1] Update `WappMetadata` struct in `host/src/loader.rs` to derive `serde::Deserialize` and match JSON schema
- [x] T003 [US1] Refactor `load_wapp` in `host/src/loader.rs` to parse 4-byte Magic, 4-byte Version, 4-byte Length, and `N`-byte JSON header

## Phase 3: User Story 2 - Set Window Title from Metadata

**Goal**: The application window title reflects the "name" field in the JSON metadata.
**Independent Verification**: Run the app and check the window title bar.

- [x] T004 [US2] Verify and update `host/src/main.rs` to ensure it uses the `name` field from the new `WappMetadata` struct (ensure field compatibility)

## Phase 4: Demo Update & Rename

**Goal**: The demo application uses the new format and is renamed to "Game of Life".

- [x] T005 Update `examples/demo/package_wapp.mjs` to generate the new JSON binary header format
- [x] T006 Rename demo output file to `game_of_life.wapp` and set JSON metadata name to "Game of Life" in `examples/demo/package_wapp.mjs`

## Phase 5: Polish & Clean Up

- [x] T007 Remove unused `read_null_terminated` helper function from `host/src/loader.rs`
- [x] T008 Verify error messages for invalid JSON or invalid header length in `host/src/loader.rs`

## Implementation Strategy
1.  **Setup**: Add dependencies first to ensure they compile.
2.  **Runtime Support**: Update the loader. This breaks the existing demo until Phase 4. This is acceptable for a feature branch.
3.  **Demo Update**: Once the host expects the new format, update the demo packager to produce it.
4.  **Verification**: Run the renamed demo to verify US1 and US2 simultaneously.
