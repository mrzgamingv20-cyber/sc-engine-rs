//! C ABI entry points called by the NativeActivity glue (C++).

use crate::ffi;
use engine_game::Game;
use std::os::raw::{c_float, c_int};
use std::sync::Mutex;

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
