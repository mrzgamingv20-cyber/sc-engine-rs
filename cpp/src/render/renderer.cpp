#include "sc/render/renderer.hpp"

#include <EGL/egl.h>
#include <GLES3/gl3.h>

#include <android/log.h>

namespace sc {
namespace render {

namespace {
constexpr int kGlEsVersionMajor = 3;
}

Renderer::~Renderer() {
    release();
}

bool Renderer::initialize(void* window, int width, int height) {
    if (initialized_) {
        return true;
    }
    window_ = window;
    width_ = width;
    height_ = height;

    display_ = eglGetDisplay(EGL_DEFAULT_DISPLAY);
    if (display_ == EGL_NO_DISPLAY) {
        __android_log_print(ANDROID_LOG_ERROR, "SCEngine", "eglGetDisplay failed");
        return false;
    }
    if (!eglInitialize(display_, nullptr, nullptr)) {
        __android_log_print(ANDROID_LOG_ERROR, "SCEngine", "eglInitialize failed");
        return false;
    }
    eglBindAPI(EGL_OPENGL_ES_API);

    const EGLint configAttribs[] = {
        EGL_RENDERABLE_TYPE, EGL_OPENGL_ES3_BIT,
        EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
        EGL_RED_SIZE, 8,
        EGL_GREEN_SIZE, 8,
        EGL_BLUE_SIZE, 8,
        EGL_ALPHA_SIZE, 8,
        EGL_DEPTH_SIZE, 24,
        EGL_NONE,
    };
    EGLConfig config = nullptr;
    EGLint numConfigs = 0;
    if (!eglChooseConfig(display_, configAttribs, &config, 1, &numConfigs) || numConfigs < 1) {
        __android_log_print(ANDROID_LOG_ERROR, "SCEngine", "eglChooseConfig failed");
        return false;
    }

    const EGLint contextAttribs[] = {EGL_CONTEXT_CLIENT_VERSION, kGlEsVersionMajor, EGL_NONE};
    context_ = eglCreateContext(display_, config, EGL_NO_CONTEXT, contextAttribs);
    if (context_ == EGL_NO_CONTEXT) {
        __android_log_print(ANDROID_LOG_ERROR, "SCEngine", "eglCreateContext failed");
        return false;
    }

    const EGLint surfaceAttribs[] = {EGL_NONE};
    surface_ = eglCreateWindowSurface(display_, config, static_cast<EGLNativeWindowType>(window_),
                                      surfaceAttribs);
    if (surface_ == EGL_NO_SURFACE) {
        __android_log_print(ANDROID_LOG_ERROR, "SCEngine", "eglCreateWindowSurface failed");
        return false;
    }
    if (!eglMakeCurrent(display_, surface_, surface_, context_)) {
        __android_log_print(ANDROID_LOG_ERROR, "SCEngine", "eglMakeCurrent failed");
        return false;
    }

    glViewport(0, 0, width_, height_);
    initialized_ = true;
    __android_log_print(ANDROID_LOG_INFO, "SCEngine", "Renderer initialized %dx%d", width_,
                        height_);
    return true;
}

void Renderer::resize(int width, int height) {
    if (!initialized_) {
        return;
    }
    width_ = width;
    height_ = height;
    glViewport(0, 0, width_, height_);
}

void Renderer::beginFrame(float r, float g, float b, float a) {
    if (!initialized_) {
        return;
    }
    glClearColor(r, g, b, a);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    // Draw a simple triangle
    static bool triangle_init = false;
    if (!triangle_init) {
        glGenBuffers(1, &triangle_vbo_);
        float vertices[] = {
            0.0f, 0.5f, 0.0f,   // top
            -0.5f, -0.5f, 0.0f, // bottom left
            0.5f, -0.5f, 0.0f   // bottom right
        };
        glGenBuffers(1, &triangle_vbo_);
        glBindBuffer(GL_ARRAY_BUFFER, triangle_vbo_);
        glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);
        triangle_init = true;
    }
    glBindBuffer(GL_ARRAY_BUFFER, triangle_vbo_);
    glEnableVertexAttribArray(0);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, 0);
    glDrawArrays(GL_TRIANGLES, 0, 3);
    glDisableVertexAttribArray(0);
}

void Renderer::endFrame() {
    if (!initialized_) {
        return;
    }
    eglSwapBuffers(display_, surface_);
}

void Renderer::uploadChunkMesh(const float* data, int length) {
    if (!initialized_ || data == nullptr || length <= 0) {
        return;
    }
    if (meshVbo_ == 0) {
        glGenBuffers(1, &meshVbo_);
    }
    glBindBuffer(GL_ARRAY_BUFFER, meshVbo_);
    const GLsizeiptr bytes = static_cast<GLsizeiptr>(length) * static_cast<GLsizeiptr>(sizeof(float));
    glBufferData(GL_ARRAY_BUFFER, bytes, data, GL_DYNAMIC_DRAW);
    meshVertexCount_ = length / 8; // Skeleton: assume 8 floats per vertex.
}

void Renderer::release() {
    if (meshVbo_ != 0) {
        glDeleteBuffers(1, &meshVbo_);
        meshVbo_ = 0;
    }
    meshVertexCount_ = 0;

    if (display_ != EGL_NO_DISPLAY) {
        eglMakeCurrent(display_, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
        if (surface_ != EGL_NO_SURFACE) {
            eglDestroySurface(display_, surface_);
        }
        if (context_ != EGL_NO_CONTEXT) {
            eglDestroyContext(display_, context_);
        }
        eglTerminate(display_);
    }

    display_ = EGL_NO_DISPLAY;
    surface_ = EGL_NO_SURFACE;
    context_ = EGL_NO_CONTEXT;
    window_ = nullptr;
    width_ = 0;
    height_ = 0;
    initialized_ = false;
}

} // namespace render
} // namespace sc
