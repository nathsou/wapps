//! WAPPS Host - WebAssembly Pixel Package Runner
//!
//! This application loads and runs WAPP packages, which contain WebAssembly
//! modules that render pixel-based graphics through SDL2.

mod graphics;
mod host_interface;
mod loader;
mod runtime;

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, error, info};
use std::path::PathBuf;
use std::time::Instant;

use graphics::Graphics;
use host_interface::HostInterface;
use runtime::WasmRuntime;

/// WAPPS Host - Run portable WebAssembly graphics applications
#[derive(Parser, Debug)]
#[command(name = "wapps")]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the .wapp file to run
    #[arg(value_name = "FILE")]
    wapp_file: PathBuf,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    // Parse CLI arguments
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .format_timestamp_millis()
        .init();

    info!("WAPPS Host starting...");
    debug!("Loading: {:?}", args.wapp_file);

    // Run the application
    if let Err(e) = run_app(&args.wapp_file) {
        error!("Application error: {:#}", e);
        std::process::exit(1);
    }

    info!("WAPPS Host shutdown complete.");
    Ok(())
}

fn run_app(wapp_path: &PathBuf) -> Result<()> {
    // Load and validate the WAPP file
    let wasm_bytes = loader::load_wapp(wapp_path)
        .with_context(|| format!("Failed to load WAPP file: {:?}", wapp_path))?;

    info!(
        "WAPP loaded successfully ({} bytes of WASM)",
        wasm_bytes.len()
    );

    // Initialize graphics
    let mut graphics = Graphics::new("WAPPS", 800, 600).context("Failed to initialize graphics")?;

    // Initialize WASM runtime with host interface
    let host_interface = HostInterface::new();
    let mut runtime = WasmRuntime::new(&wasm_bytes, host_interface)
        .context("Failed to initialize WASM runtime")?;

    // Main event loop
    let mut last_time = Instant::now();
    let target_frame_time = std::time::Duration::from_secs_f64(1.0 / 60.0);

    'main_loop: loop {
        // Calculate delta time
        let now = Instant::now();
        let dt = now.duration_since(last_time).as_secs_f64();
        last_time = now;

        // Process SDL events
        for event in graphics.poll_events() {
            use sdl2::event::Event;
            use sdl2::event::WindowEvent;

            match event {
                Event::Quit { .. } => {
                    info!("Quit event received");
                    break 'main_loop;
                }
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    debug!("Window resized to {}x{}", w, h);
                    runtime.call_on_resize(w, h)?;
                }
                Event::MouseMotion { x, y, .. } => {
                    runtime.call_on_pointer_move(x, y)?;
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => {
                    let button = mouse_button_to_int(mouse_btn);
                    runtime.call_on_pointer_down(x, y, button)?;
                }
                Event::MouseButtonUp {
                    x, y, mouse_btn, ..
                } => {
                    let button = mouse_button_to_int(mouse_btn);
                    runtime.call_on_pointer_up(x, y, button)?;
                }
                Event::KeyDown {
                    scancode: Some(sc), ..
                } => {
                    runtime.call_on_key_down(sc as i32)?;
                }
                Event::KeyUp {
                    scancode: Some(sc), ..
                } => {
                    runtime.call_on_key_up(sc as i32)?;
                }
                _ => {}
            }
        }

        // Call guest update
        runtime.call_update(dt)?;

        // Get the latest frame from the host interface and update graphics
        if let Some((width, height, pixels)) = runtime.get_frame_data() {
            graphics.update_texture(width as u32, height as u32, &pixels)?;
        }

        // Render
        graphics.render()?;

        // Frame timing
        let elapsed = Instant::now().duration_since(now);
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
    }

    Ok(())
}

fn mouse_button_to_int(btn: sdl2::mouse::MouseButton) -> i32 {
    use sdl2::mouse::MouseButton;
    match btn {
        MouseButton::Left => 1,
        MouseButton::Middle => 2,
        MouseButton::Right => 3,
        _ => 0,
    }
}
