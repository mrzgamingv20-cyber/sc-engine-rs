package com.scengine.core

object EngineBridge {
    init {
        System.loadLibrary("engine_core")
    }

    external fun nativeInit()
    external fun nativeGetVersion(): String
    external fun nativeTestMath(x: Float, y: Float, z: Float): Float
    external fun nativeLog(message: String)
}
