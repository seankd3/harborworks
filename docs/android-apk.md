# Android APK Shell

Harborworks includes a minimal Android debug APK path for Milestone 0.

This APK is a native Android Java shell. It presents the current Harborworks foundation status, the starter rescue skiff sample, and deterministic replay summary values from the headless CLI. It does not yet embed or run the Rust engine, editor, renderer, hydrodynamics, or repair workflow.

Build it with:

```sh
./tools/build-apk.sh
```

The script defaults `ANDROID_HOME` to `/home/sean/Android/Sdk`, uses the local cached Gradle binary when `gradle` is not on `PATH`, builds offline, and writes:

```text
android/app/build/outputs/apk/debug/app-debug.apk
```

Future Android work should wire the Rust runtime through an NDK/cargo-ndk path only when those tools are present and the mobile runtime boundary is explicit.

If Gradle cannot start in a restricted sandbox, the script falls back to direct Android SDK debug packaging with `aapt2`, `javac`, `d8`, `zipalign`, and `apksigner`. The fallback still produces a real signed debug APK and stores its temporary debug keystore outside the repository.
