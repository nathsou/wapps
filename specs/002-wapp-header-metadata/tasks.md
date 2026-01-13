# Tasks: WAPP Header Metadata Fields

**Input**: Design documents from `/specs/002-wapp-header-metadata/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/binary-format.md

**Tests**: Automated testing is FORBIDDEN. Do not include test tasks pursuant to Constitution Principle I.

**Organization**: Tasks are grouped by user story to enable independent implementation and verification of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

Single project structure at repository root: `host/src/`, `examples/demo/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Ensure development environment is ready

- [x] T001 Verify Rust toolchain is installed and up to date (rustc --version, cargo --version)
- [x] T002 Run cargo fmt to ensure existing code follows formatting standards
- [x] T003 Run cargo clippy to establish baseline (should pass without warnings)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data structures needed before any user story implementation

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Add WappMetadata struct to host/src/loader.rs with name and description String fields
- [x] T005 [P] Add documentation comments to WappMetadata struct explaining the binary format
- [x] T006 Add read_null_terminated helper function in host/src/loader.rs for parsing UTF-8 null-terminated strings
- [x] T007 Update WAPP_MIN_SIZE constant in host/src/loader.rs to account for minimum metadata (5 + 1 + 1 = 7 bytes)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Display App Name in Window Title (Priority: P1) 🎯 MVP

**Goal**: Parse app name from WAPP header and display it in the desktop window title bar

**Independent Verification**: Create a WAPP file with app name "My Game" and verify the window title displays "My Game"

### Implementation for User Story 1

- [x] T008 [US1] Modify load_wapp function signature in host/src/loader.rs to return (Vec<u8>, WappMetadata)
- [x] T009 [US1] Add app name parsing logic in host/src/loader.rs after version validation (call read_null_terminated with 256 byte limit)
- [x] T010 [US1] Add UTF-8 validation error handling for app name field in host/src/loader.rs
- [x] T011 [US1] Add debug logging for parsed app name in host/src/loader.rs
- [x] T012 [US1] Update load_wapp call site in host/src/main.rs to destructure (wasm_bytes, metadata)
- [x] T013 [US1] Add window title logic in host/src/main.rs: use metadata.name if not empty, otherwise use filename without extension
- [x] T014 [US1] Call SDL window.set_title() with computed title string in host/src/main.rs
- [x] T015 [US1] Create examples/demo/package_wapp.mjs script to build WAPP files with custom metadata
- [x] T016 [US1] Configure examples/demo/package_wapp.mjs with "Demo WAPP" as app name

**Manual Verification for User Story 1**:
1. Build demo: `cd examples/demo && node package_wapp.mjs`
2. Run demo: `cargo run --release -p wapps-host -- demo.wapp`
3. Verify window title shows "Demo WAPP"
4. Test with UTF-8 characters: Modify `package_wapp.mjs` constants to use "My Game 🎮" and rebuild
5. Verify emoji displays correctly in window title

**Checkpoint**: At this point, User Story 1 should be fully functional - app names display in window titles

---

## Phase 4: User Story 2 - Include App Description for Metadata (Priority: P2)

**Goal**: Parse app description from WAPP header and make it available for future use

**Independent Verification**: Create a WAPP file with description "A simple drawing application" and verify it can be extracted programmatically

### Implementation for User Story 2

- [x] T017 [US2] Add app description parsing logic in host/src/loader.rs after app name parsing (call read_null_terminated with 1024 byte limit)
- [x] T018 [US2] Add UTF-8 validation error handling for app description field in host/src/loader.rs
- [x] T019 [US2] Add debug logging for parsed app description in host/src/loader.rs
- [x] T020 [US2] Update examples/demo/package_wapp.mjs to include "A simple demonstration application" as description
- [x] T021 [US2] Verify package_wapp.mjs writes description with null terminator

**Manual Verification for User Story 2**:
1. Rebuild demo with description: `cd examples/demo && node package_wapp.mjs`
2. Run with debug logging: `RUST_LOG=debug cargo run --release -p wapps-host -- demo.wapp 2>&1 | grep description`
3. Verify description "A simple demonstration application" appears in logs
4. Modify `package_wapp.mjs` with long description (500+ chars) and verify parsing works
5. Verify description is stored in WappMetadata struct (check debug output)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work - names in titles, descriptions parsed and logged

---

## Phase 5: User Story 3 - Handle Missing or Empty Metadata (Priority: P2)

**Goal**: Gracefully handle empty metadata fields with appropriate fallback behavior

**Independent Verification**: Create WAPP files with empty name/description and verify fallback to filename works correctly

### Implementation for User Story 3

- [x] T022 [US3] Add test case in manual verification: create WAPP with empty name (just null terminator)
- [x] T023 [US3] Verify fallback logic in host/src/main.rs handles empty metadata.name correctly (uses filename without .wapp extension)
- [x] T024 [US3] Add test case: create WAPP with both empty name and empty description
- [x] T025 [US3] Verify application runs without errors when metadata fields are empty

**Manual Verification for User Story 3**:
1. Create empty metadata file by modifying `package_wapp.mjs` to use empty strings
2. Run: `cargo run --release -p wapps-host -- demo.wapp`
3. Verify window title shows "demo" (filename without extension)
4. Verify no errors or warnings in console
5. Create minimal metadata: `printf 'WAPP\x01\x00\x00' > /tmp/minimal.wapp && cat examples/demo/target/wasm32-wasip1/release/demo.wasm >> /tmp/minimal.wapp`
6. Run and verify it handles minimal header correctly

**Checkpoint**: All user stories should now be independently functional - complete feature delivered

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Error handling, documentation, and edge cases across all user stories

- [x] T026 Add comprehensive error messages for all validation failures in host/src/loader.rs
- [x] T027 [P] Verify error message for missing null terminator in app name (test with 256 bytes no null)
- [x] T028 [P] Verify error message for missing null terminator in description (test with 1024 bytes no null)
- [x] T029 [P] Verify error message for invalid UTF-8 in app name (test with 0xFF 0xFE bytes)
- [x] T030 [P] Verify error message for invalid UTF-8 in description
- [x] T031 Add file-level documentation comment to host/src/loader.rs explaining the updated binary format
- [x] T032 Update existing code comments referencing the old header format (magic + version only) to include metadata fields
- [x] T033 Run cargo fmt on all modified files
- [x] T034 Run cargo clippy and resolve any warnings in modified files
- [x] T035 Verify RUST_LOG=debug output is clear and helpful for all parsing steps

**Manual Verification - Error Cases**:
1. Wrong version: `printf 'WAPP\x02' > /tmp/bad.wapp` → expect "Unsupported WAPP version"
2. Name too long: Create 256 byte name without null → expect "exceeds maximum length"
3. Description too long: Create 1024 byte description without null → expect "exceeds maximum length"
4. Invalid UTF-8: `printf 'WAPP\x01\xFF\xFE\x00\x00' > /tmp/bad.wapp` → expect "Invalid UTF-8"
5. Truncated file: Create header-only file → expect minimum size error

---

## Dependencies Between Tasks

### Critical Path (must complete in order):
1. T001-T003 (Setup) → T004-T007 (Foundation)
2. T004-T007 (Foundation) → All user story tasks
3. T008-T016 (US1) must complete before T022-T025 (US3) since US3 tests US1's fallback behavior

### Parallel Opportunities:

**After Foundation Complete**:
- Group A (US1 Implementation): T008-T011 (loader.rs changes)
- Group B (US1 Tooling): T015-T016 (build scripts) - can run parallel with Group A
- Group C (US1 Integration): T012-T014 (main.rs changes) - depends on T008-T011

**After US1 Complete**:
- Group D (US2): T017-T021 can run in parallel with US3 verification tasks T022-T025

**Polish Phase**:
- All T026-T035 tasks can run in parallel after US3 complete

### Suggested MVP Scope (Minimum Viable Product):

**MVP = User Story 1 Only**:
- Setup: T001-T003
- Foundation: T004-T007
- US1: T008-T016
- Essential Polish: T026, T031-T034

This MVP delivers the core value: app names displayed in window titles.

---

## Implementation Strategy

### Incremental Delivery Approach:

1. **Week 1 - MVP (US1)**: Get app names working
   - Foundation + US1 implementation
   - Manual verification with test files
   - Basic error handling

2. **Week 2 - Full Feature (US1 + US2 + US3)**:
   - Add description parsing
   - Add empty metadata handling
   - Complete error handling

3. **Week 3 - Polish**:
   - Edge case validation
   - Documentation updates
   - Code quality checks

### Parallel Execution Examples:

**Example 1 - Two developers**:
- Dev A: Foundation (T004-T007) then US1 loader (T008-T011)
- Dev B: US1 tooling (T015-T016) then US1 integration (T012-T014)

**Example 2 - Single developer**:
- Day 1: Foundation (T004-T007)
- Day 2: US1 loader + tooling (T008-T011, T015-T016)
- Day 3: US1 integration + verification (T012-T014, manual tests)
- Day 4: US2 (T017-T021)
- Day 5: US3 + Polish (T022-T035)

---

## Task Summary

**Total Tasks**: 35
- Setup: 3 tasks
- Foundation: 4 tasks
- User Story 1 (P1): 9 tasks
- User Story 2 (P2): 5 tasks  
- User Story 3 (P2): 4 tasks
- Polish: 10 tasks

**Parallel Opportunities**: 15 tasks can run in parallel after dependencies met
**Critical Path**: ~20 tasks must run sequentially

**Estimated Effort**:
- Setup: 30 minutes
- Foundation: 2-3 hours
- User Story 1: 4-5 hours (including manual verification)
- User Story 2: 2-3 hours
- User Story 3: 1-2 hours (mostly verification)
- Polish: 2-3 hours

**Total Estimated Time**: 12-16 hours for complete implementation
