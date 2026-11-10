package com.scengine.core

import android.graphics.Color
import android.os.Bundle
import android.util.Log
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        EngineBridge.nativeInit()
        val version = EngineBridge.nativeGetVersion()
        val length = EngineBridge.nativeTestMath(3f, 4f, 0f)
        EngineBridge.nativeLog("MainActivity started, len(3,4,0)=$length")

        Log.i("SCEngine", "Rust engine version: $version, |(3,4,0)|=$length")

        val textView = TextView(this)
        textView.text = "SC Engine (Rust)\nVersion: $version\n| (3,4,0) | = $length"
        textView.setPadding(48, 48, 48, 48)
        textView.setTextColor(Color.WHITE)
        textView.setBackgroundColor(Color.rgb(26, 26, 46))
        textView.textSize = 20f
        setContentView(textView)
    }
}
