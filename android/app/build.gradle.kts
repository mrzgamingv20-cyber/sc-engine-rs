plugins {
    id("com.android.application")
}

android {
    namespace = "com.scengine.core"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.scengine.core"
        minSdk = 26
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"

        ndk {
            abiFilters += "arm64-v8a"
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
        }
    }
}
