# sc-engine-rs — Roadmap

Port bertahap dari `SurvivalcraftApi-API_1.9.5` (1.639 file C#) ke Rust + C++ + Assembly.
Setiap fase selesai harus: `cargo test --all` hijau, `cargo clippy --all -- -D warnings` hijau, dan APK debug bisa dibangun di CI.

## Fase 0 — Infra (sesi ini)

- [x] Dokumen arsitektur (`docs/ARCHITECTURE.md`)
- [x] Struktur repo: `cpp/`, `asm/`, `crates/engine-game`
- [x] Android NativeActivity (tanpa Kotlin/Java)
- [x] CI: test Rust + build C++/asm/Rust + APK + artifacts
- [x] Skeleton C++ renderer (EGL + GLES3 clear)
- [x] Skeleton asm NEON (simd + noise)
- [x] Skeleton `engine-game` (module tree)

**Exit:** CI hijau di `main`, APK `sc-engine-debug` ter-upload sebagai artifact.

## Fase 1 — Engine core tuntas (Rust)

Port sisa `Engine/Engine/*` ke `engine-core`:

1. `serialization` (88 file C#) — binary reader/writer, archive
2. `input` (19) — keyboard/mouse/touch/gamepad
3. `audio` (6) — mixer/sound buffer
4. `animation` (46) — keyframe/interpolation
5. `media` (56), `vr` (7) — menunggu renderer

**Exit:** semua tipe engine ter-port; test unit membandingkan nilai dengan referensi C# (golden tests).

## Fase 2 — World & content (Rust)

`Survivalcraft/Block` (304) + `ContentProvider`/`IContentReader` (24):

1. `BlockId`, `Block` registry, static block properties
2. `World` / `Chunk` / `SubChunk`, storage palette + compression
3. Content reader (XML/JSON → Rust structs), mod loading dasar
4. World gen: noise (asm `noise.S`), terrain, caves, ore

**Exit:** chunk bisa di-generate, disimpan, di-load; test golden chunk checksum.

## Fase 3 — Entity & subsystem (Rust)

`EntitySystem` (20) + `Survivalcraft/Component` (91) + `Subsystem` (111):

1. Entity, component, entity system, update order
2. Subsystem: physics, block interact, electric (51), fluida, dsb.
3. Game tick loop terintegrasi (`Game::update(dt)`)

**Exit:** simulasi berjalan headless di test (tanpa renderer), deterministik.

## Fase 4 — Renderer (C++ + Rust)

`Engine/Engine.Graphics` (69) + `Survivalcraft/Game` render-related:

1. C++ renderer: chunk mesher (dengan kernel asm), VBO/VAO, shader, frustum culling
2. Rust render command queue → C++ `sc_render_*`
3. Texture atlas, model, particle
4. Vulkan backend (opsional, setelah GLES stabil)

**Exit:** chunk world tampil di APK Android arm64.

## Fase 5 — UI & dialog (Rust + C++)

`Widget` (106), `Screen` (71), `Dialog` (57):

1. Immediate-mode UI di Rust, dirender lewat C++ batcher
2. Screen stack, dialog, inventory, pause menu

**Exit:** bisa navigasi menu in-game di APK.

## Fase 6 — Audio & input device (C++ + Rust)

1. OpenSL ES / AAudio dari C++ → Rust `audio` events
2. Touch, keyboard, gamepad via NativeActivity → Rust `input`

**Exit:** audio memutar, input berfungsi di perangkat.

## Fase 7 — Gameplay penuh & polish

1. `Survivalcraft/Game` (298) sisa: mode survival/creative, crafting, dsb.
2. `Managers` (47), `ModsManager` (72)
3. Save/load, pengaturan, lokal dsb.
4. Profiling → tambah kernel asm baru saat hotspot terbukti

**Exit:** build survivalcraft-native yang bisa dimainkan; versi rilis via CI.

## Aturan port

- Satu modul C# → satu modul Rust; jangan gabung agar diff bisa dibaca.
- Test golden: ambil nilai from C# sebagai fixture, bandingkan output Rust.
- Kernel asm hanya untuk hotspot yang terbukti (profiling), bukan spekulasi.
- Tidak ada `unsafe` di Rust kecuali FFI boundary; FFI dibungkus API aman.
- Semua build & verifikasi di GitHub CI; lokal hanya untuk edit.
