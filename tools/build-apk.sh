#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANDROID_DIR="$ROOT_DIR/android"
APK_PATH="$ANDROID_DIR/app/build/outputs/apk/debug/app-debug.apk"
MANUAL_BUILD_DIR="$ANDROID_DIR/app/build/manual-debug"
MANUAL_UNSIGNED_APK="$MANUAL_BUILD_DIR/app-unsigned.apk"
MANUAL_ALIGNED_APK="$MANUAL_BUILD_DIR/app-aligned.apk"
MANUAL_RES_ZIP="$MANUAL_BUILD_DIR/compiled-res.zip"
MANUAL_CLASSES_DIR="$MANUAL_BUILD_DIR/classes"
MANUAL_DEX_DIR="$MANUAL_BUILD_DIR/dex"
MANUAL_GENERATED_DIR="$MANUAL_BUILD_DIR/generated"
DEBUG_KEYSTORE="${DEBUG_KEYSTORE:-/tmp/harborworks-debug.keystore}"

export ANDROID_HOME="${ANDROID_HOME:-/home/sean/Android/Sdk}"
export ANDROID_SDK_ROOT="${ANDROID_SDK_ROOT:-$ANDROID_HOME}"
export PATH="$ANDROID_HOME/platform-tools:$PATH"
export GRADLE_USER_HOME="${GRADLE_USER_HOME:-/tmp/harborworks-gradle-home}"
export GRADLE_RO_DEP_CACHE="${GRADLE_RO_DEP_CACHE:-/home/sean/.gradle/caches/modules-2}"

GRADLE_BIN="${GRADLE_BIN:-}"
if [[ -z "$GRADLE_BIN" ]]; then
    if command -v gradle >/dev/null 2>&1; then
        GRADLE_BIN="$(command -v gradle)"
    else
        GRADLE_BIN="/home/sean/.gradle/wrapper/dists/gradle-8.14.3-all/10utluxaxniiv4wxiphsi49nj/gradle-8.14.3/bin/gradle"
    fi
fi

if [[ ! -x "$GRADLE_BIN" ]]; then
    echo "Gradle binary not found or not executable: $GRADLE_BIN" >&2
    exit 1
fi

PLATFORM_JAR="$ANDROID_HOME/platforms/android-35/android.jar"
BUILD_TOOLS_DIR="$ANDROID_HOME/build-tools/35.0.0"

if [[ ! -f "$PLATFORM_JAR" ]]; then
    echo "Android platform android-35 not found under $ANDROID_HOME/platforms" >&2
    exit 1
fi

if [[ ! -d "$BUILD_TOOLS_DIR" ]]; then
    echo "Android build-tools 35.0.0 not found under $ANDROID_HOME/build-tools" >&2
    exit 1
fi

mkdir -p "$GRADLE_USER_HOME"

set +e
(
    cd "$ANDROID_DIR"
    "$GRADLE_BIN" --offline --no-daemon :app:assembleDebug
)
gradle_status=$?
set -e

if [[ "$gradle_status" -ne 0 ]]; then
    echo "Gradle build failed; falling back to direct Android SDK debug packaging." >&2
    echo "This still creates a real APK with aapt2, javac, d8, zipalign, and apksigner." >&2

    rm -rf "$MANUAL_BUILD_DIR"
    mkdir -p "$MANUAL_CLASSES_DIR" "$MANUAL_DEX_DIR" "$MANUAL_GENERATED_DIR" "$(dirname "$APK_PATH")"

    "$BUILD_TOOLS_DIR/aapt2" compile \
        --dir "$ANDROID_DIR/app/src/main/res" \
        -o "$MANUAL_RES_ZIP"

    "$BUILD_TOOLS_DIR/aapt2" link \
        -I "$PLATFORM_JAR" \
        --manifest "$ANDROID_DIR/app/src/main/AndroidManifest.xml" \
        --rename-manifest-package com.seankennethdoherty.harborworks \
        --min-sdk-version 26 \
        --target-sdk-version 35 \
        --version-code 1 \
        --version-name 0.0.1-milestone0 \
        --java "$MANUAL_GENERATED_DIR" \
        -o "$MANUAL_UNSIGNED_APK" \
        "$MANUAL_RES_ZIP"

    javac \
        -source 1.8 \
        -target 1.8 \
        -bootclasspath "$PLATFORM_JAR" \
        -d "$MANUAL_CLASSES_DIR" \
        "$ANDROID_DIR/app/src/main/java/com/seankennethdoherty/harborworks/MainActivity.java"

    mapfile -t CLASS_FILES < <(find "$MANUAL_CLASSES_DIR" -name "*.class" -print)

    "$BUILD_TOOLS_DIR/d8" \
        --lib "$PLATFORM_JAR" \
        --min-api 26 \
        --output "$MANUAL_DEX_DIR" \
        "${CLASS_FILES[@]}"

    cp "$MANUAL_UNSIGNED_APK" "$MANUAL_BUILD_DIR/app-with-dex-unsigned.apk"
    (
        cd "$MANUAL_DEX_DIR"
        zip -q "$MANUAL_BUILD_DIR/app-with-dex-unsigned.apk" classes.dex
    )

    "$BUILD_TOOLS_DIR/zipalign" -f 4 "$MANUAL_BUILD_DIR/app-with-dex-unsigned.apk" "$MANUAL_ALIGNED_APK"

    if [[ ! -f "$DEBUG_KEYSTORE" ]]; then
        keytool -genkeypair \
            -keystore "$DEBUG_KEYSTORE" \
            -storepass android \
            -keypass android \
            -alias androiddebugkey \
            -keyalg RSA \
            -keysize 2048 \
            -validity 10000 \
            -dname "CN=Android Debug,O=Android,C=US" >/dev/null
    fi

    "$BUILD_TOOLS_DIR/apksigner" sign \
        --ks "$DEBUG_KEYSTORE" \
        --ks-key-alias androiddebugkey \
        --ks-pass pass:android \
        --key-pass pass:android \
        --out "$APK_PATH" \
        "$MANUAL_ALIGNED_APK"
fi

if [[ ! -f "$APK_PATH" ]]; then
    echo "APK build completed but expected output was not found: $APK_PATH" >&2
    exit 1
fi

echo "APK built: $APK_PATH"
