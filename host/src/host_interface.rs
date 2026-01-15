//! Host Interface
//!
//! Manages the shared state between the WASM guest and the host application.
//! Stores the latest frame data from update_frame calls.
//!
//! Performance: Uses a pre-allocated buffer that is reused across frames
//! to avoid heap allocations on every update_frame call.

/// Host interface for communication between WASM guest and host
pub struct HostInterface {
    /// Latest frame width
    frame_width: i32,
    /// Latest frame height  
    frame_height: i32,
    /// Reusable pixel buffer (RGBA format) - avoids allocation per frame
    frame_buffer: Vec<u8>,
    /// Actual length of valid data in frame_buffer
    frame_len: usize,
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
            frame_len: 0,
            frame_dirty: false,
        }
    }

    /// Store a new frame from the guest
    ///
    /// Performance: Reuses existing buffer capacity when possible,
    /// only reallocates if the new frame is larger than current capacity.
    pub fn set_frame(&mut self, width: i32, height: i32, pixels: &[u8]) {
        self.frame_width = width;
        self.frame_height = height;
        self.frame_len = pixels.len();
        
        // Reuse buffer if capacity is sufficient, otherwise grow
        if self.frame_buffer.capacity() < pixels.len() {
            // Reserve exact capacity to avoid over-allocation
            self.frame_buffer.reserve(pixels.len() - self.frame_buffer.capacity());
        }
        
        // Resize to exact length (cheap if capacity exists)
        self.frame_buffer.resize(pixels.len(), 0);
        
        // Copy pixel data into reused buffer
        self.frame_buffer.copy_from_slice(pixels);
        self.frame_dirty = true;
    }

    /// Borrow the latest frame data if available (clears the dirty flag)
    ///
    /// Returns a reference to the internal buffer, avoiding ownership transfer.
    /// The caller should use this data immediately before the next set_frame call.
    pub fn borrow_frame(&mut self) -> Option<(i32, i32, &[u8])> {
        if self.frame_dirty {
            self.frame_dirty = false;
            Some((self.frame_width, self.frame_height, &self.frame_buffer[..self.frame_len]))
        } else {
            None
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
