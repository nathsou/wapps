# Implementation Plan: JSON Header Format Refactor

**Branch**: `003-json-header-format` | **Date**: 2026-01-15 | **Spec**: [specs/003-json-header-format/spec.md](spec.md)
**Input**: Feature specification from `/specs/003-json-header-format/spec.md`

## Summary

Refactor the WAPP binary file format to support extensible JSON metadata. The new format consists of a 4-byte Magic Sequence ("WAPP"), a 4-byte Version (1), a 4-byte Header Length, followed by the JSON metadata string of that length, and finally the WASM binary. This replaces the previous fixed-layout null-terminated string format. The runtime will parse this JSON header and use the "name" field (if present) to set the application window title.

## Technical Context

**Language/Version**: Rust (Stable)
**Primary Dependencies**: 
- `serde` & `serde_json` (New: for metadata parsing)
- `byteorder` (Optional, or use std for little-endian handling)
- `anyhow` (Existing: error handling)
- `sdl2` (Existing: graphics/windowing)
**Storage**: N/A
**Testing**: Strictly FORBIDDEN per Constitution (Manual verification only)
**Target Platform**: Desktop (macOS, Linux, Windows), WASM (Runtime)
**Project Type**: Single binary (Host)
**Performance Goals**: Fast startup (parsing JSON header should be negligible).
**Constraints**: Header size virtually unlimited (u32), but practically constrained by memory.
**Scale/Scope**: Core file format change, affects `loader.rs` and `main.rs`.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Code Quality**: Adheres to strict type safety (using `serde` for typed parsing). No automated tests.
- **Performance**: JSON parsing overhead is minimal for metadata.
- **Readability**: Structured JSON is more readable/standard than ad-hoc binary formats.
- **Simplicity**: `serde_json` is a standard dependency, avoiding custom parsing logic.

## Project Structure

### Documentation (this feature)

```text
specs/003-json-header-format/
├── plan.md              # This file
├── research.md          # Technology choices (Phase 0)
├── data-model.md        # Binary format spec (Phase 1)
├── quickstart.md        # Usage guide (Phase 1)
├── contracts/           # N/A
└── tasks.md             # Implementation tasks (Phase 2)
```

### Source Code (repository root)

```text
host/
├── Cargo.toml           # Add serde/serde_json
└── src/
    ├── loader.rs        # Rewrite load_wapp for new format
    └── main.rs          # Update metadata usage
```

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Dependency: `serde`/`serde_json` | To robustly parse arbitrary JSON metadata as required by spec. | Custom JSON parser is error-prone and reinvents the wheel. `serde` is the ecosystem standard. |
