# sc-engine-rs

Survivalcraft Engine core — Rust port.

A Rust rewrite of the core engine math and utility types from [SurvivalcraftApi](https://github.com/SC-SPM), targeting Android arm64 as a native `cdylib`.

## Crates

| Crate | Description |
|-------|-------------|
| `engine-core` | Core math types (Vector2/3/4, Matrix, Quaternion, Color, geometry), MathUtils, Log, Time |

## Building

```bash
# Native
cargo build

# Android arm64 (requires cargo-ndk + Android NDK)
cargo install cargo-ndk
cargo ndk -t arm64-v8a -o ./jniLibs build --release
```

## Testing

```bash
cargo test --all
```

## CI

GitHub Actions builds and tests on every push. The `build-android-arm64` job produces a `.so` library for `aarch64-linux-android`.
