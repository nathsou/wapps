//! WASM Runtime
//!
//! Manages the Wasmtime engine, WASI context, and module instantiation.
//! Configures minimal WASI capabilities for security (clock, random, stdio only).

use anyhow::{bail, Context, Result};
use log::{debug, warn};
use std::sync::{Arc, Mutex};
use wasmtime::*;
use wasmtime_wasi::preview1::{self, WasiP1Ctx};
use wasmtime_wasi::WasiCtxBuilder;

use crate::host_interface::HostInterface;

/// Combined state for the WASM store
pub struct StoreState {
    /// WASI context for system calls
    wasi: WasiP1Ctx,
    /// Host interface for graphics
    host: Arc<Mutex<HostInterface>>,
}

impl StoreState {
    fn new(host: HostInterface) -> Self {
        // Configure minimal WASI - security restricted:
        // - Inherit stdout/stderr for debugging
        // - Allow basic time/random access
        // - NO file system access
        // - NO network access
        // - NO environment variables
        let wasi = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            // Note: clock and random are enabled by default in WASI
            // File system is NOT inherited - sandboxed
            .build_p1();

        Self {
            wasi,
            host: Arc::new(Mutex::new(host)),
        }
    }
}

/// WASM Runtime manages the Wasmtime execution environment
#[allow(dead_code)]
pub struct WasmRuntime {
    store: Store<StoreState>,
    instance: Instance,
    // Cached function handles for exports
    update_fn: Option<TypedFunc<f64, ()>>,
    on_resize_fn: Option<TypedFunc<(i32, i32), ()>>,
    on_pointer_move_fn: Option<TypedFunc<(i32, i32), ()>>,
    on_pointer_down_fn: Option<TypedFunc<(i32, i32, i32), ()>>,
    on_pointer_up_fn: Option<TypedFunc<(i32, i32, i32), ()>>,
    on_key_down_fn: Option<TypedFunc<i32, ()>>,
    on_key_up_fn: Option<TypedFunc<i32, ()>>,
    // Memory reference for frame data access
    memory: Memory,
    // Shared host interface
    host_interface: Arc<Mutex<HostInterface>>,
}

impl WasmRuntime {
    /// Create a new WASM runtime and instantiate the given module
    pub fn new(wasm_bytes: &[u8], host_interface: HostInterface) -> Result<Self> {
        // Create engine with default configuration
        let engine = Engine::default();

        // Create store with combined state
        let host_arc = {
            let state = StoreState::new(host_interface);
            let arc = state.host.clone();
            let mut store = Store::new(&engine, state);

            // Configure trap handler for graceful error reporting
            store.set_epoch_deadline(1);

            (store, arc)
        };

        let (mut store, host_arc_clone) = host_arc;

        // Create linker and add WASI functions
        let mut linker: Linker<StoreState> = Linker::new(&engine);

        // Add WASI Preview 1 sync functions
        preview1::add_to_linker_sync(&mut linker, |state: &mut StoreState| &mut state.wasi)
            .context("Failed to add WASI functions to linker")?;

        // Add our host import: wapps::update_frame
        linker
            .func_wrap(
                "wapps",
                "update_frame",
                |mut caller: Caller<'_, StoreState>, width: i32, height: i32, pixels_ptr: i32| {
                    let memory = caller
                        .get_export("memory")
                        .and_then(|e| e.into_memory())
                        .expect("Guest must export 'memory'");

                    let data = memory.data(&caller);
                    let ptr = pixels_ptr as usize;
                    let len = (width * height * 4) as usize;

                    if ptr + len > data.len() {
                        warn!("update_frame: pixel buffer out of bounds");
                        return;
                    }

                    let pixels = &data[ptr..ptr + len];

                    // Store frame data in host interface
                    if let Ok(mut host) = caller.data().host.lock() {
                        host.set_frame(width, height, pixels);
                    }
                },
            )
            .context("Failed to register update_frame import")?;

        // Compile the module
        debug!("Compiling WASM module...");
        let module = Module::new(&engine, wasm_bytes).context("Failed to compile WASM module")?;

        // Instantiate
        debug!("Instantiating WASM module...");
        let instance = linker
            .instantiate(&mut store, &module)
            .context("Failed to instantiate WASM module")?;

        // Get memory export (required)
        let memory = instance
            .get_memory(&mut store, "memory")
            .context("Guest must export 'memory'")?;

        // Cache optional export functions
        let update_fn = instance
            .get_typed_func::<f64, ()>(&mut store, "update")
            .ok();

        let on_resize_fn = instance
            .get_typed_func::<(i32, i32), ()>(&mut store, "on_resize")
            .ok();

        let on_pointer_move_fn = instance
            .get_typed_func::<(i32, i32), ()>(&mut store, "on_pointer_move")
            .ok();

        let on_pointer_down_fn = instance
            .get_typed_func::<(i32, i32, i32), ()>(&mut store, "on_pointer_down")
            .ok();

        let on_pointer_up_fn = instance
            .get_typed_func::<(i32, i32, i32), ()>(&mut store, "on_pointer_up")
            .ok();

        let on_key_down_fn = instance
            .get_typed_func::<i32, ()>(&mut store, "on_key_down")
            .ok();

        let on_key_up_fn = instance
            .get_typed_func::<i32, ()>(&mut store, "on_key_up")
            .ok();

        // Verify required export exists
        if update_fn.is_none() {
            bail!("Guest must export 'update(dt: f64)' function");
        }

        debug!("WASM module instantiated successfully");
        debug!("  - update: present");
        debug!(
            "  - on_resize: {}",
            if on_resize_fn.is_some() {
                "present"
            } else {
                "absent"
            }
        );
        debug!(
            "  - on_pointer_move: {}",
            if on_pointer_move_fn.is_some() {
                "present"
            } else {
                "absent"
            }
        );
        debug!(
            "  - on_pointer_down: {}",
            if on_pointer_down_fn.is_some() {
                "present"
            } else {
                "absent"
            }
        );
        debug!(
            "  - on_pointer_up: {}",
            if on_pointer_up_fn.is_some() {
                "present"
            } else {
                "absent"
            }
        );
        debug!(
            "  - on_key_down: {}",
            if on_key_down_fn.is_some() {
                "present"
            } else {
                "absent"
            }
        );
        debug!(
            "  - on_key_up: {}",
            if on_key_up_fn.is_some() {
                "present"
            } else {
                "absent"
            }
        );

        Ok(Self {
            store,
            instance,
            update_fn,
            on_resize_fn,
            on_pointer_move_fn,
            on_pointer_down_fn,
            on_pointer_up_fn,
            on_key_down_fn,
            on_key_up_fn,
            memory,
            host_interface: host_arc_clone,
        })
    }

    /// Call the guest's update function
    pub fn call_update(&mut self, dt: f64) -> Result<()> {
        if let Some(func) = &self.update_fn {
            func.call(&mut self.store, dt)
                .context("Error calling guest 'update' function")?;
        }
        Ok(())
    }

    /// Call the guest's on_resize function (if present)
    pub fn call_on_resize(&mut self, width: i32, height: i32) -> Result<()> {
        if let Some(func) = &self.on_resize_fn {
            func.call(&mut self.store, (width, height))
                .context("Error calling guest 'on_resize' function")?;
        }
        Ok(())
    }

    /// Call the guest's on_pointer_move function (if present)
    pub fn call_on_pointer_move(&mut self, x: i32, y: i32) -> Result<()> {
        if let Some(func) = &self.on_pointer_move_fn {
            func.call(&mut self.store, (x, y))
                .context("Error calling guest 'on_pointer_move' function")?;
        }
        Ok(())
    }

    /// Call the guest's on_pointer_down function (if present)
    pub fn call_on_pointer_down(&mut self, x: i32, y: i32, button: i32) -> Result<()> {
        if let Some(func) = &self.on_pointer_down_fn {
            func.call(&mut self.store, (x, y, button))
                .context("Error calling guest 'on_pointer_down' function")?;
        }
        Ok(())
    }

    /// Call the guest's on_pointer_up function (if present)
    pub fn call_on_pointer_up(&mut self, x: i32, y: i32, button: i32) -> Result<()> {
        if let Some(func) = &self.on_pointer_up_fn {
            func.call(&mut self.store, (x, y, button))
                .context("Error calling guest 'on_pointer_up' function")?;
        }
        Ok(())
    }

    /// Call the guest's on_key_down function (if present)
    pub fn call_on_key_down(&mut self, scancode: i32) -> Result<()> {
        if let Some(func) = &self.on_key_down_fn {
            func.call(&mut self.store, scancode)
                .context("Error calling guest 'on_key_down' function")?;
        }
        Ok(())
    }

    /// Call the guest's on_key_up function (if present)
    pub fn call_on_key_up(&mut self, scancode: i32) -> Result<()> {
        if let Some(func) = &self.on_key_up_fn {
            func.call(&mut self.store, scancode)
                .context("Error calling guest 'on_key_up' function")?;
        }
        Ok(())
    }

    /// Process the latest frame data from the host interface
    ///
    /// Calls the provided closure with the frame data (width, height, pixels slice)
    /// if a new frame is available. This avoids copying the pixel data.
    pub fn with_frame_data<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(i32, i32, &[u8]) -> R,
    {
        let mut host = self.host_interface.lock().ok()?;
        if let Some((w, h, pixels)) = host.borrow_frame() {
            Some(f(w, h, pixels))
        } else {
            None
        }
    }
}
