# sc-engine-rs

Survivalcraft — native rewrite in Rust, C++, and AArch64 assembly.

Android arm64 target: a single `cdylib` (`libengine_app.so`) containing the Rust
engine + game logic, the C++ renderer/platform layer, and NEON assembly kernels.
No Kotlin/Java code ships in the APK; the activity is `android.app.NativeActivity`.

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) and [docs/ROADMAP.md](docs/ROADMAP.md).

| Layer | Language | Location |
|---|---|---|
| Engine + game logic | Rust | `crates/engine-core`, `crates/engine-game` |
| Android app entry + FFI | Rust | `crates/engine-app` |
| Renderer + platform glue | C++ | `cpp/` |
| Hot kernels (SIMD, noise) | Assembly (NEON) | `asm/` |

## Crates

| Crate | Description |
|---|---|
| `engine-core` | Core math, serialization, input, audio, time, utilities |
| `engine-game` | Blocks, world, entities, subsystems, game tick |
| `engine-app` | Android `cdylib`: C ABI entry points, FFI to C++/asm |

## Building

All builds run in GitHub Actions (see `.github/workflows/ci.yml`).

- `test` job: `cargo test --all` + `cargo clippy --all --all-targets -- -D warnings` on Linux.
- `build-android` job: `cargo ndk -t arm64-v8a` builds the Rust cdylib and, via
  `build.rs`, compiles `cpp/` + `asm/` with the NDK into `libsc_render.a` and
  links it into `libengine_app.so`. Gradle then packages the APK.

Artifacts: `sc-engine-debug-apk`, `engine-app-android-arm64` (`.so`), and
`native-build-artifacts` (static libs).

## Local development

```bash
cargo test --all
cargo clippy --all --all-targets -- -D warnings
```

Android builds need the NDK and `ANDROID_NDK_HOME`; CI is the source of truth.

## Testing

```bash
cargo test --all
```

## CI

GitHub Actions builds and tests on every push. The `build-android` job produces
an APK for `arm64-v8a` plus the native libraries as artifacts.
