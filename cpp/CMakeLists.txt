cmake_minimum_required(VERSION 3.22)
project(sc_render LANGUAGES C CXX ASM)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

set(SC_ASM_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../asm")

# NativeActivity glue: compiled straight into libsc_render.a so that the
# `app_dummy` reference from android_main.cpp drags in the object that also
# defines ANativeActivity_onCreate. Keeping it as a separate CMake library
# would leave those symbols out of libengine_app.so (Rust only links
# libsc_render.a).
if(ANDROID)
    set(SC_PLATFORM_SOURCES
        "${ANDROID_NDK}/sources/android/native_app_glue/android_native_app_glue.c")
    set(SC_PLATFORM_INCLUDES
        "${ANDROID_NDK}/sources/android/native_app_glue")
else()
    set(SC_PLATFORM_SOURCES "")
    set(SC_PLATFORM_INCLUDES "")
endif()

add_library(sc_render STATIC
    src/render/renderer.cpp
    src/platform/android_main.cpp
    ${SC_PLATFORM_SOURCES}
    "${SC_ASM_DIR}/simd.S"
    "${SC_ASM_DIR}/noise.S")

target_include_directories(sc_render PUBLIC
    "${CMAKE_CURRENT_SOURCE_DIR}/include"
    "${SC_PLATFORM_INCLUDES}"
    "${SC_ASM_DIR}/include")

if(ANDROID)
    target_link_libraries(sc_render PUBLIC log android EGL GLESv3)
endif()