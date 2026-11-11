#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ---------------------------------------------------------------------------
 * Rust -> C++ renderer API.
 * Implemented in cpp/src/platform/android_main.cpp (Renderer).
 * ------------------------------------------------------------------------- */

int  sc_render_init(void* window, int width, int height);
void sc_render_resize(int width, int height);
void sc_render_begin_frame(float r, float g, float b, float a);
void sc_render_end_frame(void);
void sc_render_upload_chunk_mesh(const float* data, int length);
void sc_render_release(void);

/* ---------------------------------------------------------------------------
 * C++ platform layer -> Rust game API.
 * Implemented in crates/engine-app (android.rs).
 * ------------------------------------------------------------------------- */

void sc_game_on_ready(void);
void sc_game_tick(float dt);
void sc_game_on_surface_lost(void);
void sc_game_on_input(int type, float x, float y, int button);

#ifdef __cplusplus
}
#endif
