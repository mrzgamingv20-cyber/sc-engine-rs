//! JNI bridge for Android. Exposes engine-core functions to Kotlin/Java.

use jni::objects::{JClass, JString};
use jni::sys::{jfloat, jstring};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_scengine_core_EngineBridge_nativeInit(
    _env: JNIEnv,
    _class: JClass,
) {
    crate::log::init();
    crate::time::init();
    crate::log::write(
        crate::log::LogType::Information,
        "SC Engine initialized (Rust)",
    );
}

#[no_mangle]
pub extern "system" fn Java_com_scengine_core_EngineBridge_nativeGetVersion(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    env.new_string("0.1.0-rust")
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "system" fn Java_com_scengine_core_EngineBridge_nativeTestMath(
    _env: JNIEnv,
    _class: JClass,
    x: jfloat,
    y: jfloat,
    z: jfloat,
) -> jfloat {
    let v = crate::Vector3::new(x, y, z);
    v.length()
}

#[no_mangle]
pub extern "system" fn Java_com_scengine_core_EngineBridge_nativeLog(
    mut env: JNIEnv,
    _class: JClass,
    message: JString,
) {
    let msg: String = env
        .get_string(&message)
        .map(|s| s.into())
        .unwrap_or_default();
    crate::log::write(crate::log::LogType::Information, &msg);
}
