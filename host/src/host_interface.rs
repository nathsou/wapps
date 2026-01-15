//! Host Interface
//!
//! Manages the shared state between the WASM guest and the host application.
//! Stores the latest frame data from update_frame calls.

/// Host interface for communication between WASM guest and host
pub struct HostInterface {
    /// Latest frame width
    frame_width: i32,
    /// Latest frame height  
    frame_height: i32,
    /// Latest pixel data (RGBA format)
    frame_buffer: Vec<u8>,
    /// Flag indicating new frame data is available
    frame_dirty: bool,
}

impl HostInterface {
    /// Create a new host interface
    pub fn new() -> Self {
        Self {
            frame_width: 0,
            frame_height: 0,
            frame_buffer: Vec::new(),
            frame_dirty: false,
        }
    }

    /// Store a new frame from the guest
    pub fn set_frame(&mut self, width: i32, height: i32, pixels: &[u8]) {
        self.frame_width = width;
        self.frame_height = height;

        // Resize buffer if needed, reusing allocation
        if self.frame_buffer.len() != pixels.len() {
            self.frame_buffer.resize(pixels.len(), 0);
        }
        self.frame_buffer.copy_from_slice(pixels);
        
        self.frame_dirty = true;
    }

    /// Access the latest frame data if available (consumes the dirty flag).
    /// Returns true if a frame was processed, false otherwise.
    pub fn with_frame<F>(&mut self, f: F) -> bool
    where
        F: FnOnce(i32, i32, &[u8]),
    {
        if self.frame_dirty {
            self.frame_dirty = false;
            f(self.frame_width, self.frame_height, &self.frame_buffer);
            true
        } else {
            false
        }
    }

    /// Get the current frame dimensions
    #[allow(dead_code)]
    pub fn frame_dimensions(&self) -> (i32, i32) {
        (self.frame_width, self.frame_height)
    }
}

impl Default for HostInterface {
    fn default() -> Self {
        Self::new()
    }
}
