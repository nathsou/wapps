# Feature Specification: WAPP Web Host Environment

**Feature Branch**: `004-web-host-env`
**Created**: 2026-01-15
**Status**: Draft
**Input**: User description: "Create a new WAPP host environment: the Web, it should behave identically to the desktop platform"

## User Scenarios & Verification

### User Story 1 - Load and Run WAPP in Browser (Priority: P1)

As a user, I want to open a web page and load a local `.wapp` file so that I can run the application without installing native software.

**Why this priority**: Core functionality of the feature. Without this, the web host does not exist.

**Independent Verification**: Can be verified by opening the deployed web host page and loading a known working WAPP file.

**Acceptance Scenarios**:

1. **Given** the user is on the WAPP Web Host page, **When** they drag and drop a valid `.wapp` file, **Then** the application starts running in the display area.
2. **Given** the user is on the WAPP Web Host page, **When** they use the file picker to select a valid `.wapp` file, **Then** the application starts running in the display area.
3. **Given** a running WAPP, **When** the internal logic updates the display (e.g. animation), **Then** the browser display updates to reflect the new frame.

---

### User Story 2 - Interact with WAPP (Priority: P1)

As a user, I want to use my mouse and keyboard to interact with the running WAPP so that I can control the application.

**Why this priority**: Essential for interactivity; most WAPPs are not just passive animations.

**Independent Verification**: Can be verified by using a WAPP that responds to input (e.g. `game_of_life.wapp` toggling cells).

**Acceptance Scenarios**:

1. **Given** a running WAPP, **When** the user clicks on the display area, **Then** the WAPP receives the mouse click event with correct coordinates.
2. **Given** a running WAPP, **When** the user presses a key, **Then** the WAPP receives the keyboard event.
3. **Given** a running WAPP, **When** the user moves the mouse, **Then** the WAPP receives cursor position updates (if requested).

### Edge Cases

- What happens when an invalid file is loaded? (Should show an error message).
- How does the system handle browser resizing? (Display should likely resize to fit or maintain aspect ratio).
- What happens if the browser doesn't support the required web technologies? (Should show a polite error).
- Performance on mobile devices? (Touch events mapped to mouse events?).

## Requirements

### Functional Requirements

- **FR-001**: The system MUST provide a web-accessible interface to interpret and execute `.wapp` files.
- **FR-002**: The system MUST implement the WAPP Host Interface specifications on the web platform, ensuring compatibility with existing WAPP binaries.
- **FR-003**: The system MUST render the application's pixel buffer to the web page display area.
- **FR-004**: The system MUST capture standard browser input events (mousedown, mouseup, mousemove, keydown, keyup) and translate them into the WAPP input event format.
- **FR-005**: The system MUST support a frame loop mechanism synchronized with the browser's display refresh rate to drive the WAPP execution cycle.
- **FR-006**: The system MUST provide feedback if loading fails (e.g. corrupt file or invalid format).

### Key Entities

- **Web Host Runtime**: The logic that mimics the native host's role in the browser.
- **WAPP Executable**: The user-provided file containing the application.
- **Virtual Input Device**: The abstraction layer mapping browser events to WAPP events.
- **Frame Buffer**: The shared memory region for graphics.

## Success Criteria

### Measurable Outcomes

- **SC-001**: `game_of_life.wapp` runs in the web host with identical logic behavior to the desktop version.
- **SC-002**: Rendering maintains a steady frame rate (target > 30 FPS for standard lightweight apps).
- **SC-003**: Input latency is low enough for interactive use (response to click visible in next frame).
- **SC-004**: Can load and run a WAPP file entirely client-side without server-side processing of the file logic.