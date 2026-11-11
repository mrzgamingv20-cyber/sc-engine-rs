//! Android application crate for sc-engine-rs.
//!
//! This is the only `cdylib` in the workspace. It links the C++/assembly layer
//! (see `cpp/` and `asm/`) and exposes the C ABI entry points called from the
//! NativeActivity glue in `cpp/src/platform/android_main.cpp`.

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
mod ffi;
