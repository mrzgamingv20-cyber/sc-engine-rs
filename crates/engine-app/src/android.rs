//! C ABI entry points called by the NativeActivity glue (C++).

use crate::ffi;
use engine_game::Game;
use std::os::raw::{c_float, c_int, c_void};
use std::sync::Mutex;

extern "C" {
    /// `ANativeActivity_onCreate` from the NDK native_app_glue, renamed in
    /// CMake (`ANativeActivity_onCreate=sc_glue_on_create`) so that the wrapper
    /// below can own the real symbol.
    fn sc_glue_on_create(activity: *mut c_void, saved_state: *mut c_void, saved_state_size: usize);
}

/// Android calls this right after `dlopen`-ing the library.
///
/// It has to be exported from a Rust object: rustc passes `--exclude-libs` when
/// linking a `cdylib`, which hides every symbol that comes from a static
/// library (the C++ renderer and the NDK glue). Defining the entry point here
/// keeps it in the dynamic symbol table.
#[no_mangle]
pub extern "C" fn ANativeActivity_onCreate(
    activity: *mut c_void,
    saved_state: *mut c_void,
    saved_state_size: usize,
) {
    // SAFETY: called by the Android runtime with a valid ANativeActivity*.
    unsafe { sc_glue_on_create(activity, saved_state, saved_state_size) }
}

/// Game instance owned by the Android thread.
///
/// The C++ `android_main` calls into this crate on the native app thread, so a
/// global mutex is enough for now.
static GAME: Mutex<Option<Game>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn sc_game_on_ready() {
    let mut slot = GAME.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if slot.is_none() {
        *slot = Some(Game::new());
    }
}

#[no_mangle]
pub extern "C" fn sc_game_tick(dt: c_float) {
    let mut slot = GAME.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(game) = slot.as_mut() {
        game.update(dt);
    }
    ffi::begin_frame(0.05, 0.07, 0.12, 1.0);
    ffi::end_frame();
}

#[no_mangle]
pub extern "C" fn sc_game_on_surface_lost() {
    // TODO(Fase 4): pause simulation and release Rust-owned render resources.
}

#[no_mangle]
pub extern "C" fn sc_game_on_input(_type: c_int, _x: c_float, _y: c_float, _button: c_int) {
    // TODO(Fase 6): forward to the engine-core input state.
}
