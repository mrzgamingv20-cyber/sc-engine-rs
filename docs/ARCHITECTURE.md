# sc-engine-rs — Arsitektur

Tujuan: rewrite Survivalcraft (game) menjadi native Android arm64 menggunakan **hanya Rust, C++, dan Assembly**.
Sources of truth: `~/survivalcraft-src/SurvivalcraftApi-API_1.9.5` (1.639 file C#, ~235k baris).

## 1. Peran bahasa

| Bahasa | Tanggung jawab | Lokasi |
|---|---|---|
| **Rust** | Seluruh logika engine + game (math, world, block, entity, subsystem, game loop, input, audio, serialization) | `crates/engine-core`, `crates/engine-game` |
| **C++** | Render/graphics (OpenGL ES 3, nanti Vulkan), platform glue (NativeActivity, EGL, event loop, input forwarding) | `cpp/` |
| **Assembly (AArch64/NEON)** | Kernel panas: SIMD math, noise/hash, chunk meshing, compress/decompress | `asm/` |

Tidak ada Kotlin/Java/C# di APK. Shell Android memakai `android.app.NativeActivity`; semua logika platform ditulis di C++.

## 2. Struktur repo

```
sc-engine-rs/
├── Cargo.toml                  # workspace: engine-core, engine-game, engine-app
├── crates/
│   ├── engine-core/            # Rust (rlib): math, serialization, input, audio, time, log
│   ├── engine-game/            # Rust (rlib): block, world, entity, subsystem, game
│   └── engine-app/             # Rust (cdylib): entry point Android + FFI
│       ├── build.rs            # Android: cmake (cpp/ + asm/) → libsc_render.a → link
│       └── src/{lib,ffi,android}.rs
├── cpp/
│   ├── CMakeLists.txt          # lib statis sc_render (+ native_app_glue + asm)
│   ├── include/sc/ffi.h        # batas C ABI Rust ↔ C++
│   ├── include/sc/render/      # renderer.hpp
│   └── src/
│       ├── render/             # EGL/GLES3 renderer
│       └── platform/           # android_main + sc_render_* glue (NativeActivity)
├── asm/
│   ├── include/sc/asm/         # simd.h, noise.h
│   ├── simd.S                  # NEON f32x4 add/mul/dot
│   └── noise.S                 # NEON hash 4 lane (world gen)
├── android/
│   ├── app/src/main/
│   │   ├── AndroidManifest.xml # NativeActivity, hasCode=false
│   │   └── res/                # resources + theme sistem
│   └── build.gradle.kts        # tanpa Kotlin/AndroidX
└── .github/workflows/ci.yml    # test Rust, build C+++asm+Rust, APK, artifacts
```

## 3. Batas Rust ↔ C++ (FFI)

C ABI di `cpp/include/sc/ffi.h`:

```c
/* Rust -> C++ */
int   sc_render_init(void* window, int width, int height);
void  sc_render_resize(int width, int height);
void  sc_render_begin_frame(float r, float g, float b, float a);
void  sc_render_end_frame(void);
void  sc_render_upload_chunk_mesh(const float* data, int length);
void  sc_render_release(void);

/* C++ -> Rust */
void  sc_game_on_ready(void);
void  sc_game_tick(float dt);
void  sc_game_on_surface_lost(void);
void  sc_game_on_input(int type, float x, float y, int button);
```

Rust mendeklarasikan `extern "C"` di `crates/engine-app/src/ffi.rs` (hanya `cfg(target_os = "android")`).
Alur data satu arah: C++ platform → Rust tick; Rust → C++ render command.

## 4. Alur runtime

```
NativeActivity (framework)
  └─ android_main (C++, native_app_glue)
       ├─ APP_CMD_INIT_WINDOW → Renderer::initialize(ANativeWindow) + sc_game_on_ready()
       ├─ loop:
       │    ├─ ALooper_pollAll → sc_game_on_input(...)
       │    └─ sc_game_tick(dt)  [Rust: simulasi + sc_render_begin/end_frame]
       └─ APP_CMD_TERM_WINDOW / DESTROY → sc_render_release()
```

VSYNC/Choreographer dan frame pacing datang di Fase 4 (loop saat ini cap ~60 FPS dengan sleep).

## 5. Build

- **Host (CI `test`)**: `cargo test --all`, `cargo clippy --all --all-targets -- -D warnings`.
- **Android (CI `build-android`)**: `cargo ndk -t arm64-v8a` → `build.rs` menjalankan CMake + NDK toolchain (`android.toolchain.cmake`) → `libsc_render.a` (C++ + asm, PIC) → di-link ke `libengine_app.so` (beserta `c++_static`, `log`, `android`, `EGL`, `GLESv3`) → Gradle pack APK.
- Artifact: APK + `libengine_app.so` + `libsc_render.a`.

Catatan linker: `sc_render_*` dan `android_main` sengaja dalam satu translation unit (`android_main.cpp`) agar reference Rust menarik object yang berisi entry point NativeActivity. Kernel asm yang belum direferensi belum masuk `.so` (linker membuang object tak terpakai); saat modul terkait dipakai, otomatis ikut.

## 6. Porting C# → Rust (peta modul)

| C# (`SurvivalcraftApi-API_1.9.5`) | Jumlah file | Target Rust |
|---|---:|---|
| `Engine/Engine` (math, core) | 40 | `engine-core` (sebagian sudah) |
| `Engine/Engine.Graphics` | 69 | C++ (`cpp/`) + Rust wrapper |
| `Engine/Engine.Serialization` | 88 | `engine-core::serialization` (sebagian sudah) |
| `Engine/Engine.Audio` | 6 | `engine-core::audio` (sebagian sudah) |
| `Engine/Engine.Input` | 19 | `engine-core::input` (sebagian sudah) |
| `Engine/Engine.Media`, `.Animation`, `.VR` | 109 | `engine-core` / `engine-game` |
| `EntitySystem` | 20 | `engine-game::entity` |
| `Survivalcraft/Block` | 304 | `engine-game::block` |
| `Survivalcraft/Game` | 298 | `engine-game::game` |
| `Survivalcraft/Subsystem` | 111 | `engine-game::subsystem` |
| `Survivalcraft/Component` | 91 | `engine-game::entity::component` |
| `Survivalcraft/Widget`, `Screen`, `Dialog` | 234 | Rust UI (nanti) / C++ renderer |
| `Survivalcraft/Managers`, `ModsManager` | 119 | `engine-game::managers` |
| `Survivalcraft/ElectricElement` | 51 | `engine-game::electric` |
| `Survivalcraft/ContentProvider`, `IContentReader` | 24 | `engine-game::content` |
| `Survivalcraft/Animation` | 29 | `engine-game::animation` |

## 7. Kernel Assembly (tahap awal)

- `asm/simd.S`: `sc_simd_add_f32x4`, `sc_simd_mul_f32x4`, `sc_simd_dot_f32x4` (NEON).
- `asm/noise.S`: `sc_noise_hash_u32x4` (finalizer xxHash-style, 4 lane sekaligus).
- Kernel berikutnya (mesher, chunk compress, frustum) ditulis saat modul terkait di-port, setelah profiling membuktikan hotspot.

## 8. Keputusan & trade-off

- **NativeActivity, bukan Activity+GLSurfaceView**: memenuhi “hanya Rust/C++/Assembly”; tidak ada dex di APK.
- **C++ + asm static-linked ke cdylib Rust**: satu `.so` (`libengine_app.so`), tidak ada urutan load antar-library, linkage sederhana.
- **Belum ada Vulkan**: mulai GLES3 (minSdk 26, `glEsVersion=0x00030002`), Vulkan menyusul saat renderer stabil.
- **Build only via GitHub CI** (sesuai permintaan): semua artifact (test result, `.so`, APK) dari Actions.
- **`unsafe` hanya di boundary FFI**: `engine-game` memakai `#![forbid(unsafe_code)]`; wrapper FFI dibungkus API aman.
