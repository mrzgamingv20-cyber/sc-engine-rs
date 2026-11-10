# Keep JNI bridge
-keep class com.scengine.core.EngineBridge { *; }
-keepclasseswithmembernames class * {
    native <methods>;
}
