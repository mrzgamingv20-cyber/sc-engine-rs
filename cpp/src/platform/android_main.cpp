#include <android_native_app_glue.h>
#include <android/log.h>

#include <chrono>
#include <thread>

#include "sc/ffi.h"
#include "sc/render/renderer.hpp"

// Defined by the NDK's android_native_app_glue; calling it forces the linker to
// keep the glue object, which also provides ANativeActivity_onCreate.
extern "C" void app_dummy();

namespace {

sc::render::Renderer g_renderer;
bool g_windowReady = false;

// ---------------------------------------------------------------------------
// Rust -> C++ renderer API. Kept in this translation unit (together with
// android_main) so that Rust's references pull the object that contains the
// NativeActivity entry point into the shared library.
// ---------------------------------------------------------------------------

} // namespace

extern "C" int sc_render_init(void* window, int width, int height) {
    return g_renderer.initialize(window, width, height) ? 0 : -1;
}

extern "C" void sc_render_resize(int width, int height) {
    g_renderer.resize(width, height);
}

extern "C" void sc_render_begin_frame(float r, float g, float b, float a) {
    g_renderer.beginFrame(r, g, b, a);
}

extern "C" void sc_render_end_frame() {
    g_renderer.endFrame();
}

extern "C" void sc_render_upload_chunk_mesh(const float* data, int length) {
    g_renderer.uploadChunkMesh(data, length);
}

extern "C" void sc_render_release() {
    g_renderer.release();
}

namespace {

void handleAppCmd(android_app* app, int32_t cmd) {
    switch (cmd) {
        case APP_CMD_INIT_WINDOW:
            if (app->window != nullptr) {
                const int width = ANativeWindow_getWidth(app->window);
                const int height = ANativeWindow_getHeight(app->window);
                if (sc_render_init(app->window, width, height) == 0) {
                    g_windowReady = true;
                    sc_game_on_ready();
                } else {
                    __android_log_print(ANDROID_LOG_ERROR, "SCEngine", "renderer init failed");
                }
            }
            break;
        case APP_CMD_TERM_WINDOW:
            g_windowReady = false;
            sc_game_on_surface_lost();
            sc_render_release();
            break;
        case APP_CMD_WINDOW_RESIZED:
            if (app->window != nullptr && g_windowReady) {
                sc_render_resize(ANativeWindow_getWidth(app->window),
                                 ANativeWindow_getHeight(app->window));
            }
            break;
        case APP_CMD_DESTROY:
            g_windowReady = false;
            sc_render_release();
            break;
        default:
            break;
    }
}

int32_t handleInputEvent(android_app* app, AInputEvent* event) {
    (void)app;
    if (AInputEvent_getType(event) == AINPUT_EVENT_TYPE_MOTION) {
        const int32_t action = AMotionEvent_getAction(event) & AMOTION_EVENT_ACTION_MASK;
        const int type = action == AMOTION_EVENT_ACTION_DOWN
                             ? 0
                             : action == AMOTION_EVENT_ACTION_MOVE
                                   ? 1
                                   : action == AMOTION_EVENT_ACTION_UP ? 2 : 3;
        const float x = AMotionEvent_getX(event, 0);
        const float y = AMotionEvent_getY(event, 0);
        sc_game_on_input(type, x, y, 0);
        return 1;
    }
    return 0;
}

} // namespace

void android_main(android_app* app) {
#if defined(__clang__)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#endif
    // Keep the native_app_glue object (and thus ANativeActivity_onCreate) in the
    // final .so. Deprecated in r27 but still the cheapest way to force that.
    app_dummy();
#if defined(__clang__)
#pragma clang diagnostic pop
#endif

    app->onAppCmd = handleAppCmd;
    app->onInputEvent = handleInputEvent;

    using clock = std::chrono::steady_clock;
    auto previous = clock::now();

    while (true) {
        // Drain all pending events without blocking (ALooper_pollAll was removed
        // from the NDK headers in r27; pollOnce with a 0 timeout is the
        // non-blocking equivalent).
        int ident = 0;
        int events = 0;
        android_poll_source* source = nullptr;
        while ((ident = ALooper_pollOnce(0, nullptr, &events,
                                         reinterpret_cast<void**>(&source)) >= 0)) {
            if (source != nullptr) {
                source->process(source->app, source);
            }
            if (app->destroyRequested != 0) {
                sc_render_release();
                return;
            }
        }

        if (g_windowReady) {
            const auto now = clock::now();
            const float dt = std::chrono::duration<float>(now - previous).count();
            previous = now;
            sc_game_tick(dt);
        }

        // Skeleton frame pacing (~60 FPS). Replace with Choreographer/vsync in Fase 4.
        std::this_thread::sleep_for(std::chrono::milliseconds(16));
    }
}
