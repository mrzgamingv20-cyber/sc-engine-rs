use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var("TARGET").unwrap_or_default();
    if !target.contains("android") {
        // Host builds do not link the C++/assembly layer.
        return;
    }

    let ndk = env::var("ANDROID_NDK_HOME")
        .or_else(|_| env::var("NDK_HOME"))
        .expect("ANDROID_NDK_HOME (or NDK_HOME) must be set for Android builds");
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");
    println!("cargo:rerun-if-env-changed=NDK_HOME");

    let ndk = PathBuf::from(ndk);
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir
        .parent()
        .and_then(|crate_dir| crate_dir.parent())
        .expect("engine-app must live in crates/");
    let cpp_dir = workspace_root.join("cpp");
    let asm_dir = workspace_root.join("asm");
    println!("cargo:rerun-if-changed={}", cpp_dir.display());
    println!("cargo:rerun-if-changed={}", asm_dir.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("sc-render-build");
    let toolchain = ndk.join("build/cmake/android.toolchain.cmake");
    assert!(
        toolchain.exists(),
        "NDK CMake toolchain not found: {}",
        toolchain.display()
    );

    let status = Command::new("cmake")
        .arg("-S")
        .arg(&cpp_dir)
        .arg("-B")
        .arg(&build_dir)
        .arg(format!("-DCMAKE_TOOLCHAIN_FILE={}", toolchain.display()))
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg("-DANDROID_ABI=arm64-v8a")
        .arg("-DANDROID_PLATFORM=android-26")
        .arg(format!("-DANDROID_NDK={}", ndk.display()))
        .status()
        .expect("failed to run cmake configure");
    assert!(status.success(), "cmake configure failed");

    let status = Command::new("cmake")
        .arg("--build")
        .arg(&build_dir)
        .arg("--config")
        .arg("Release")
        .arg("--parallel")
        .status()
        .expect("failed to run cmake build");
    assert!(status.success(), "cmake build failed");

    println!("cargo:rustc-link-search=native={}", build_dir.display());
    println!("cargo:rustc-link-lib=static=sc_render");
    // Keep the NativeActivity entry point even though nothing in Rust calls it:
    // Android looks it up via dlopen/dlsym. Without --undefined the linker is
    // free to drop the object (and --gc-sections would happily do so).
    println!("cargo:rustc-link-arg=-Wl,--undefined=ANativeActivity_onCreate");
    println!("cargo:rustc-link-lib=static=c++_static");
    println!("cargo:rustc-link-lib=log");
    println!("cargo:rustc-link-lib=android");
    println!("cargo:rustc-link-lib=EGL");
    println!("cargo:rustc-link-lib=GLESv3");
}
