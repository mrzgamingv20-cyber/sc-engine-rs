//! Raw FFI declarations for the native C++/assembly layer.
//!
//! Only compiled for `target_os = "android"`; the `sc_render` static library is
//! linked by `build.rs` on Android builds.

use std::os::raw::{c_float, c_int, c_void};

#[allow(dead_code)]
extern "C" {
    // C++ renderer (cpp/src/platform/android_main.cpp)
    pub fn sc_render_init(window: *mut c_void, width: c_int, height: c_int) -> c_int;
    pub fn sc_render_resize(width: c_int, height: c_int);
    pub fn sc_render_begin_frame(r: c_float, g: c_float, b: c_float, a: c_float);
    pub fn sc_render_end_frame();
    pub fn sc_render_upload_chunk_mesh(data: *const c_float, length: c_int);
    pub fn sc_render_release();

    // Assembly kernels (asm/simd.S, asm/noise.S)
    pub fn sc_simd_add_f32x4(a: *const c_float, b: *const c_float, out: *mut c_float);
    pub fn sc_simd_mul_f32x4(a: *const c_float, b: *const c_float, out: *mut c_float);
    pub fn sc_simd_dot_f32x4(a: *const c_float, b: *const c_float) -> c_float;
    pub fn sc_noise_hash_u32x4(input: *const u32, output: *mut u32);
}

pub fn begin_frame(r: f32, g: f32, b: f32, a: f32) {
    unsafe { sc_render_begin_frame(r, g, b, a) }
}

pub fn end_frame() {
    unsafe { sc_render_end_frame() }
}
