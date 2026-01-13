//! Graphics Module
//!
//! Handles SDL2 window creation, texture management, and rendering.
//! Uses streaming textures for efficient pixel buffer updates.

use anyhow::{Context, Result};
use log::debug;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;
use sdl2::Sdl;

/// Graphics manager handling SDL2 window and rendering
pub struct Graphics {
    #[allow(dead_code)]
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    texture: Option<Texture<'static>>,
    event_pump: EventPump,
    current_width: u32,
    current_height: u32,
    needs_render: bool,
}

impl Graphics {
    /// Create a new graphics context with an SDL2 window
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
        debug!("Initializing SDL2...");

        let sdl_context =
            sdl2::init().map_err(|e| anyhow::anyhow!("Failed to initialize SDL2: {}", e))?;

        let video_subsystem = sdl_context
            .video()
            .map_err(|e| anyhow::anyhow!("Failed to initialize video subsystem: {}", e))?;

        debug!("Creating window {}x{}", width, height);

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .resizable()
            .build()
            .context("Failed to create window")?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .context("Failed to create canvas")?;

        let texture_creator = canvas.texture_creator();

        let event_pump = sdl_context
            .event_pump()
            .map_err(|e| anyhow::anyhow!("Failed to get event pump: {}", e))?;

        debug!("Graphics initialized successfully");

        Ok(Self {
            sdl_context,
            canvas,
            // SAFETY: texture_creator lifetime is tied to canvas which we own
            #[allow(clippy::useless_transmute)]
            texture_creator: unsafe { std::mem::transmute(texture_creator) },
            texture: None,
            event_pump,
            current_width: width,
            current_height: height,
            needs_render: true,
        })
    }

    /// Poll for SDL events
    pub fn poll_events(&mut self) -> Vec<Event> {
        self.event_pump.poll_iter().collect()
    }

    /// Update the texture with new pixel data
    ///
    /// Reuses the existing texture if dimensions match.
    /// Pixel format: RGBA (4 bytes per pixel)
    pub fn update_texture(&mut self, width: u32, height: u32, pixels: &[u8]) -> Result<()> {
        // Check if we need to recreate the texture
        if self.texture.is_none() || width != self.current_width || height != self.current_height {
            debug!("Creating new texture {}x{}", width, height);
            self.current_width = width;
            self.current_height = height;

            // Resize window to match content
            let (win_w, win_h) = self.canvas.window().size();
            if win_w != width || win_h != height {
                let _ = self.canvas.window_mut().set_size(width, height);
            }

            // Create new streaming texture
            let texture = self
                .texture_creator
                .create_texture_streaming(PixelFormatEnum::RGBA32, width, height)
                .context("Failed to create streaming texture")?;

            // SAFETY: texture lifetime is managed manually, texture_creator outlives texture
            self.texture =
                Some(unsafe { std::mem::transmute::<Texture<'_>, Texture<'static>>(texture) });
        }

        // Update texture with pixel data
        if let Some(ref mut texture) = self.texture {
            let pitch = (width * 4) as usize;
            texture
                .update(None, pixels, pitch)
                .map_err(|e| anyhow::anyhow!("Failed to update texture: {}", e))?;
        }

        self.needs_render = true;
        Ok(())
    }

    /// Render the current frame to screen
    pub fn render(&mut self) -> Result<()> {
        if !self.needs_render && self.texture.is_some() {
            // No changes, skip render
            return Ok(());
        }

        // Clear with black
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Copy texture if available
        if let Some(ref texture) = self.texture {
            self.canvas
                .copy(texture, None, None)
                .map_err(|e| anyhow::anyhow!("Failed to copy texture: {}", e))?;
        }

        // Present
        self.canvas.present();
        self.needs_render = false;

        Ok(())
    }
}
