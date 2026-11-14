#pragma once

#include <cstdint>

namespace sc {
namespace render {

/**
 * OpenGL ES 3 renderer owning the EGL surface and the GL resources.
 *
 * The renderer is driven from the Android native app thread: the C++ platform
 * layer calls initialize/release on window lifecycle events, and the Rust game
 * loop calls begin_frame/end_frame once per tick.
 */
class Renderer {
public:
    Renderer() = default;
    ~Renderer();

    Renderer(const Renderer&) = delete;
    Renderer& operator=(const Renderer&) = delete;

    /** Create the EGL surface/context on @p window. Returns true on success. */
    bool initialize(void* window, int width, int height);

    /** Update the drawing buffer size. */
    void resize(int width, int height);

    /** Clear the framebuffer with the given color. */
    void beginFrame(float r, float g, float b, float a);

    /** Swap the back buffer to the screen. */
    void endFrame();

    /** Upload a chunk mesh as a float vertex stream (skeleton). */
    void uploadChunkMesh(const float* data, int length);

    /** Release all GL/EGL resources. Safe to call more than once. */
    void release();

    bool isInitialized() const { return initialized_; }

private:
    void* window_ = nullptr;
    int width_ = 0;
    int height_ = 0;
    void* display_ = nullptr;
    void* surface_ = nullptr;
    void* context_ = nullptr;
    unsigned int meshVbo_ = 0;
    unsigned int triangleVbo_ = 0;
    int meshVertexCount_ = 0;
    bool initialized_ = false;
};

} // namespace render
} // namespace sc
