# Tasks: WAPP Web Host Environment

**Feature Branch**: `004-web-host-env`
**Status**: Ready for Implementation

## Phase 1: Setup

Goal: Initialize the project structure for the web host (Pure JS).

- [x] T001 Create `host/web` directory structure (index.html, styles.css, main.js, runtime.js)
- [x] T002 Vendor `@bjorn3/browser_wasi_shim` by creating a build script or manual download to `host/web/vendor/`
- [x] T003 Create `host/web/index.html` with basic layout and canvas

## Phase 2: Foundational

Goal: Implement the core JS Runtime and WASI shim integration (blocking for all user stories).

- [x] T004 Implement `WappRuntime` class skeleton in `host/web/runtime.js`
- [x] T005 Implement `imports` object creation in `runtime.js` merging `wasi_snapshot_preview1` (from shim) and `wapps` (host interface)
- [x] T006 Implement `WappRuntime.load(bytes)` to instantiate WebAssembly module with imports
- [x] T007 Implement `wapps.update_frame` callback in `runtime.js` to store frame pointer and dimensions

## Phase 3: User Story 1 - Load and Run WAPP

Goal: Allow users to load a .wapp file and see it running in the browser.

- [x] T008 [US1] Implement `WappRuntime.start()` to run the requestAnimationFrame loop calling `exports.update(dt)`
- [x] T009 [US1] Implement rendering logic in `runtime.js` (reading memory buffer -> ImageData -> Canvas)
- [x] T010 [P] [US1] Implement File Picker and Drag-and-Drop listeners in `host/web/main.js`
- [x] T011 [US1] Wire up `main.js` file loading to `WappRuntime.load()` and `start()`

## Phase 4: User Story 2 - Interact with WAPP

Goal: Enable Mouse and Keyboard interaction.

- [x] T012 [P] [US2] Implement `handleMouseEvent` in `WappRuntime` class calling `on_pointer_*` exports
- [x] T013 [P] [US2] Implement `handleKeyEvent` in `WappRuntime` class calling `on_key_*` exports
- [x] T014 [US2] Add mouse event listeners to Canvas in `host/web/main.js` and call runtime
- [x] T015 [US2] Add keyboard event listeners to Window in `host/web/main.js` and call runtime

## Phase 5: Polish & CI/CD

Goal: Deployment and clean up.

- [x] T016 Create GitHub Action `web-host-deploy.yml` for deploying `host/web` to GitHub Pages
- [x] T017 Verify `game_of_life.wapp` behavior in the deployed environment
- [x] T018 Add error handling UI in `host/web/index.html` for invalid files or shim errors

## Dependencies

- Phase 2 (Runtime) blocks Phase 3 and 4.
- Phase 3 (Loading) blocks Phase 4 (Interaction) from being fully testable, though implementation of T012/T013 can proceed in parallel.

## Parallel Execution Examples

- T010 (Launcher UI) can be built in parallel with T004-T007 (Runtime Logic).
- T012 and T013 (Input Handlers) can be coded before the full event loop is ready.