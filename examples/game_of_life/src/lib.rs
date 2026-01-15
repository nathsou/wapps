//! Demo WAPP Application - Conway's Game of Life
//!
//! This is a demo guest application that implements Conway's Game of Life.
//! It demonstrates the WAPP interface by rendering a pixel-based simulation.

use std::cell::RefCell;

// ============================================================================
// Host Imports (from "wapps" module)
// ============================================================================

#[link(wasm_import_module = "wapps")]
extern "C" {
    /// Update the host display with pixel data
    fn update_frame(width: i32, height: i32, pixels_ptr: *const u8);
}

// ============================================================================
// Game State
// ============================================================================

/// Canvas dimensions
const WIDTH: usize = 200;
const HEIGHT: usize = 150;
const CELL_SIZE: usize = 4; // Each cell is 4x4 pixels

/// Total pixel dimensions
const PIXEL_WIDTH: usize = WIDTH * CELL_SIZE;
const PIXEL_HEIGHT: usize = HEIGHT * CELL_SIZE;

/// RGBA pixel buffer
const BUFFER_SIZE: usize = PIXEL_WIDTH * PIXEL_HEIGHT * 4;

// Game state stored in thread-local storage
thread_local! {
    static STATE: RefCell<GameState> = const { RefCell::new(GameState::new()) };
}

struct GameState {
    /// Current generation grid
    cells: [[bool; WIDTH]; HEIGHT],
    /// Next generation grid (double buffer)
    next_cells: [[bool; WIDTH]; HEIGHT],
    /// Pixel buffer for rendering
    pixels: [u8; BUFFER_SIZE],
    /// Time accumulator for simulation steps
    time_accum: f64,
    /// Simulation step interval (seconds)
    step_interval: f64,
    /// Whether simulation is paused
    paused: bool,
    /// Whether left mouse button is held down (for drawing)
    drawing: bool,
}

impl GameState {
    const fn new() -> Self {
        Self {
            cells: [[false; WIDTH]; HEIGHT],
            next_cells: [[false; WIDTH]; HEIGHT],
            pixels: [0; BUFFER_SIZE],
            time_accum: 0.0,
            step_interval: 0.1, // 10 steps per second
            paused: false,
            drawing: false,
        }
    }

    /// Initialize with a random pattern using a simple LCG
    fn randomize(&mut self) {
        let mut seed: u32 = 12345;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                // Simple LCG random
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                self.cells[y][x] = (seed >> 16) & 1 == 1;
            }
        }
    }

    /// Add a glider pattern at the specified position
    fn add_glider(&mut self, x: usize, y: usize) {
        if x + 2 < WIDTH && y + 2 < HEIGHT {
            // Glider pattern
            //   X
            //     X
            // X X X
            self.cells[y][x + 1] = true;
            self.cells[y + 1][x + 2] = true;
            self.cells[y + 2][x] = true;
            self.cells[y + 2][x + 1] = true;
            self.cells[y + 2][x + 2] = true;
        }
    }

    /// Count live neighbors for a cell
    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0u8;

        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (x as i32 + dx).rem_euclid(WIDTH as i32) as usize;
                let ny = (y as i32 + dy).rem_euclid(HEIGHT as i32) as usize;

                if self.cells[ny][nx] {
                    count += 1;
                }
            }
        }

        count
    }

    /// Perform one simulation step
    fn step(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let neighbors = self.count_neighbors(x, y);
                let alive = self.cells[y][x];

                // Conway's rules:
                // - Live cell with 2 or 3 neighbors survives
                // - Dead cell with exactly 3 neighbors becomes alive
                self.next_cells[y][x] =
                    matches!((alive, neighbors), (true, 2) | (true, 3) | (false, 3));
            }
        }

        // Swap buffers
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    /// Render cells to pixel buffer
    fn render(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let alive = self.cells[y][x];

                // Cell color: green if alive, dark if dead
                let (r, g, b) = if alive {
                    (50, 205, 50) // Lime green
                } else {
                    (20, 20, 30) // Dark blue-gray
                };

                // Fill cell pixels
                for cy in 0..CELL_SIZE {
                    for cx in 0..CELL_SIZE {
                        let px = x * CELL_SIZE + cx;
                        let py = y * CELL_SIZE + cy;
                        let idx = (py * PIXEL_WIDTH + px) * 4;

                        self.pixels[idx] = r;
                        self.pixels[idx + 1] = g;
                        self.pixels[idx + 2] = b;
                        self.pixels[idx + 3] = 255; // Alpha
                    }
                }
            }
        }
    }

    /// Set cell alive at pixel coordinates
    fn set_alive_at_pixel(&mut self, px: i32, py: i32) {
        if px >= 0 && py >= 0 {
            let x = (px as usize) / CELL_SIZE;
            let y = (py as usize) / CELL_SIZE;

            if x < WIDTH && y < HEIGHT {
                self.cells[y][x] = true;
            }
        }
    }

    /// Toggle cell at pixel coordinates
    fn toggle_at_pixel(&mut self, px: i32, py: i32) {
        if px >= 0 && py >= 0 {
            let x = (px as usize) / CELL_SIZE;
            let y = (py as usize) / CELL_SIZE;

            if x < WIDTH && y < HEIGHT {
                self.cells[y][x] = !self.cells[y][x];
            }
        }
    }
}

// ============================================================================
// Guest Exports
// ============================================================================

/// Main update function called ~60 times per second
#[no_mangle]
pub extern "C" fn update(dt: f64) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();

        // Initialize on first frame
        static mut INITIALIZED: bool = false;
        unsafe {
            if !INITIALIZED {
                state.randomize();
                // Add some gliders for visual interest
                state.add_glider(10, 10);
                state.add_glider(50, 30);
                state.add_glider(100, 60);
                INITIALIZED = true;
            }
        }

        // Accumulate time and step simulation
        if !state.paused {
            state.time_accum += dt;
            while state.time_accum >= state.step_interval {
                state.step();
                state.time_accum -= state.step_interval;
            }
        }

        // Render to pixel buffer
        state.render();

        // Send frame to host
        unsafe {
            update_frame(
                PIXEL_WIDTH as i32,
                PIXEL_HEIGHT as i32,
                state.pixels.as_ptr(),
            );
        }
    });
}

/// Called when window is resized
#[no_mangle]
pub extern "C" fn on_resize(_width: i32, _height: i32) {
    // For this demo, we ignore resize and keep fixed dimensions
    // A more advanced app could adjust its simulation grid
}

/// Called when pointer/mouse moves
#[no_mangle]
pub extern "C" fn on_pointer_move(x: i32, y: i32) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.drawing {
            state.set_alive_at_pixel(x, y);
        }
    });
}

/// Called when pointer/mouse button is pressed
#[no_mangle]
pub extern "C" fn on_pointer_down(x: i32, y: i32, button: i32) {
    if button == 1 {
        // Left click: start drawing and add alive cell
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.drawing = true;
            state.set_alive_at_pixel(x, y);
        });
    }
}

/// Called when pointer/mouse button is released
#[no_mangle]
pub extern "C" fn on_pointer_up(_x: i32, _y: i32, button: i32) {
    if button == 1 {
        // Left release: stop drawing
        STATE.with(|state| {
            state.borrow_mut().drawing = false;
        });
    }
}

/// Called when a key is pressed
#[no_mangle]
pub extern "C" fn on_key_down(scancode: i32) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();

        // Space (scancode 44) = pause/resume
        if scancode == 44 {
            state.paused = !state.paused;
        }
        // R (scancode 21) = randomize
        else if scancode == 21 {
            state.randomize();
        }
        // C (scancode 6) = clear
        else if scancode == 6 {
            state.cells = [[false; WIDTH]; HEIGHT];
        }
    });
}

/// Called when a key is released
#[no_mangle]
pub extern "C" fn on_key_up(_scancode: i32) {
    // Not used in this demo
}
