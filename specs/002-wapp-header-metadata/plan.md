# Implementation Plan: WAPP Header Metadata Fields

**Branch**: `002-wapp-header-metadata` | **Date**: 2026-01-13 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-wapp-header-metadata/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Extend the WAPP binary file format to include UTF-8 encoded metadata fields (app name and description) in the header. The app name will be displayed in the desktop window title bar, and the description will be available for future launchers/file inspectors. Both fields are null-terminated strings with strict byte length limits (256 bytes for name, 1024 bytes for description). The implementation requires modifications to the existing loader module to parse and validate these new fields while maintaining the same version byte (0x01).

## Technical Context

**Language/Version**: Rust (Edition 2021, implied stable toolchain)
**Primary Dependencies**: 
- wasmtime 29 (WASM runtime)
- wasmtime-wasi 29 (WASI support)
- sdl2 0.37 (graphics, window management)
- anyhow 1 (error handling)
**Storage**: Binary file format (.wapp files) - no database
**Testing**: Strictly FORBIDDEN per Constitution (manual verification only)
**Target Platform**: Desktop (macOS, Linux, Windows) via SDL2
**Project Type**: Single project (host runtime)
**Performance Goals**: 
- Header parsing must complete in <1ms
- Window title update within 100ms of application startup
- No performance degradation for existing 60 FPS rendering loop
**Constraints**: 
- UTF-8 validation must be strict (reject invalid sequences)
- Maximum header size bounded (5 + 256 + 1024 = 1285 bytes maximum)
- Backward compatibility not required (same version byte, all files now have metadata)
**Scale/Scope**: 
- Small feature scope: 1-2 files modified (loader.rs, possibly main.rs)
- ~100-200 lines of code added/modified

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**I. Code Quality** ✅
- No automated testing required (Constitution forbids)
- Will use Rust's type safety and Result types for error handling
- UTF-8 validation via std library (zero-copy when possible)
- Clear error messages for validation failures

**II. Performance** ✅
- Header parsing is one-time operation at startup
- UTF-8 validation is O(n) with early termination on invalid sequences
- No impact on rendering loop (parsed once at load time)

**III. Readability** ✅
- Clear function signatures for parse operations
- Descriptive variable names for metadata fields
- Documentation comments for binary format structure

**IV. Simplicity** ✅
- Uses only std library for UTF-8 validation (no new dependencies)
- Straightforward sequential parsing (read magic, version, name, description, wasm)
- No complex state machines or abstractions needed

**Violations**: None

**Assessment**: PASS - Proceed to Phase 0

## Project Structure

### Documentation (this feature)

```text
specs/002-wapp-header-metadata/
├── spec.md              # Feature specification (input)
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (technology research)
├── data-model.md        # Phase 1 output (binary format structure)
├── quickstart.md        # Phase 1 output (developer guide)
├── contracts/
│   └── binary-format.md # Phase 1 output (format contract)
├── checklists/
│   └── requirements.md  # Quality validation checklist
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT YET CREATED)
```

### Source Code (repository root)

```text
host/
├── Cargo.toml           # Dependencies: wasmtime, sdl2, anyhow
└── src/
    ├── loader.rs        # MODIFIED: Add metadata parsing
    ├── main.rs          # MODIFIED: Set window title from metadata
    ├── host_interface.rs # No changes
    ├── graphics.rs       # No changes
    └── runtime.rs        # No changes

examples/demo/
├── package_wapp.mjs     # NEW: Orchestration script (builds WASM + creates header)
├── Cargo.toml           # No changes
└── src/
    └── lib.rs           # No changes

demo.wapp                # REGENERATED: Include metadata in header
```

**Structure Decision**: Single project structure. This feature only modifies the existing host runtime. Changes are isolated to:
1. `host/src/loader.rs` - Add metadata parsing functions (~100 lines)
2. `host/src/main.rs` - Set window title from metadata (~10 lines)
3. `examples/demo/package_wapp.mjs` - New orchestration script (builds WASM + creates header)

No new modules, no architectural changes, no additional dependencies.

## Complexity Tracking

No Constitution violations - tracking table not needed.
