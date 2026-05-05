# Harborworks

A code-only high-performance engineering sandbox for building, testing, breaking, repairing, and using custom marine rescue/work boats.

Primary target: PC desktop prototype.

See `harborworks_mvp_spec.md` for the MVP specification.

## Milestone 0 Foundation

This repository currently contains a code-only Rust foundation, not a completed game or MVP.

Workspace crates:

- `hw_core`: typed IDs, shared diagnostics, schema constants.
- `hw_math`: serde-friendly vector/transform and unit wrappers.
- `hw_vehicle`: editable blueprint model and validation.
- `hw_vehicle_compile`: deterministic compiler scaffold from blueprint to runtime proxy.
- `hw_sim`: deterministic headless replay stub.
- `hw_systems`: placeholder boundary for future electrical/fluid/control systems.
- `hw_tools`: CLI validation and replay commands.

Useful commands:

```sh
cargo run -p hw_tools -- validate
cargo run -p hw_tools -- sim-replay --blueprint examples/blueprints/starter_rescue_skiff.json --seconds 10
./tools/check.sh
```

Android debug APK shell:

```sh
./tools/build-apk.sh
```

The APK is written to `android/app/build/outputs/apk/debug/app-debug.apk`. It is a Milestone 0 Android shell, not yet the Rust engine/editor port.
