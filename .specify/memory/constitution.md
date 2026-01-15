<!--
Sync Impact Report:
- Version Change: 1.0.0 -> 2.0.0 (Forbidden Testing & Strict Dependency Minimization)
- Modified Principles:
    - Code Quality: Added strict ban on automated testing.
    - Simplicity: Added strict minimal dependency requirement.
- Added Sections: N/A
- Removed Sections: N/A
- Templates Requiring Updates:
    - .specify/templates/plan-template.md (Updated Testing field)
    - .specify/templates/tasks-template.md (Updated Tests guidance)
    - .specify/templates/spec-template.md (Renamed Testing -> Verification)
- TODOs: None
-->

# wapps Constitution

## Core Principles

### I. Code Quality
Code MUST be robust and maintainable. Automated testing (unit, integration, e2e) is strictly FORBIDDEN. Quality is guaranteed through strict adherence to type safety, compiler guarantees, and manual verification. Technical debt MUST be addressed immediately when encountered ("Boy Scout Rule"). Errors MUST be handled explicitly; panic/crashing is not acceptable for user errors.

### II. Performance
Systems MUST be optimized for low latency and efficient resource usage. Critical paths SHOULD be benchmarked. Premature optimization is discouraged, but architectural decisions MUST consider performance implications from the start.

### III. Readability
Code is read more often than it is written. Variable and function names MUST be descriptive and unambiguous. Consistency in style and formatting is MANDATORY. Comments should explain the "why", not the "how". Complex logic MUST be documented.

### IV. Simplicity
Adhere to KISS (Keep It Simple, Stupid) and YAGNI (You Aren't Gonna Need It). Minimal dependencies are MANDATORY; prefer the standard library over external crates unless absolutely necessary. Essential complexity is allowed; accidental complexity MUST be eliminated. Simpler solutions are preferred over clever ones.

## Technical Implementation

### Stack & Standards
- **Language**: Use the most appropriate language for the task ensuring idiomatic usage.
- **Formatting**: Standard formatting tools for the language MUST be used.
- **Linting**: Strict linting MUST pass without warnings.

## Development Workflow

### Review & Quality
- All changes MUST be reviewed via Pull Request.
- CI/CD pipelines MUST pass (formatting, linting) before merge (NO tests).
- Commits SHOULD be atomic and semantic (Conventional Commits).

## Governance

This constitution governs the development of the wapps project. Amendments require a Pull Request with updated versioning and rationale. All contributors must adhere to these principles.

**Version**: 2.1.0 | **Ratified**: 2026-01-13 | **Last Amended**: 2026-01-15
