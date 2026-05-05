# Harborworks Codex Report

Date: 2026-05-05

## Implemented

- Created a small Rust Cargo workspace with `hw_core`, `hw_math`, `hw_vehicle`, `hw_vehicle_compile`, `hw_sim`, `hw_systems`, and `hw_tools`.
- Added crate README/module docs with responsibilities and non-responsibilities.
- Added strong typed IDs for nodes, beams, panels, components, ports, routes, compartments, materials, and parts.
- Added serde-friendly math wrappers and basic unit newtypes.
- Implemented editable blueprint structs, deterministic sorting normalization, JSON load/save, and validation for duplicate IDs, missing references, finite positions/transforms, and non-negative numeric values.
- Added material/beam part definitions plus validation.
- Added initial JSON schemas, material assets, part asset, and `starter_rescue_skiff` example blueprint.
- Implemented blueprint compiler scaffolding that computes placeholder beam/component mass, center of mass, inertia placeholder, and warnings.
- Implemented deterministic headless replay stub and CLI commands:
  - `cargo run -p hw_tools -- validate`
  - `cargo run -p hw_tools -- sim-replay --blueprint examples/blueprints/starter_rescue_skiff.json --seconds 10`
- Added `tools/check.sh`.

## Not Implemented

- No graphical editor, renderer, UI, missions, real CAD, mesh generation, real physics, real hydrodynamics, real buoyancy, FEA, damage, repair, or full system simulation.
- JSON schemas are initial hand-authored contract scaffolds; CLI validation currently parses schemas and validates data through Rust typed models rather than performing full JSON Schema validation.
- Runtime vehicle is an honest deterministic proxy only.

## Commands Run

- `cargo fmt --all`: passed.
- `cargo test --workspace --offline`: passed after removing uncached `schemars_derive` usage.
- `cargo fmt --all --check`: passed.
- `cargo test --workspace`: passed.
- `cargo run -p hw_tools -- validate`: passed.
  - Starter runtime proxy mass: `55.888 kg`.
  - Starter center of mass: `(0.000, 0.593, -0.518)`.
- `cargo run -p hw_tools -- sim-replay --blueprint examples/blueprints/starter_rescue_skiff.json --seconds 10`: passed.
  - Ticks: `100`.
  - Final position: `(0.000, 0.000, 662.288)`.
  - Final speed: `101.978 m/s`.
  - Energy placeholder: `814614.875 J`.
- `./tools/check.sh`: passed.

Note: the first plain Cargo attempt tried to refresh `index.crates.io` and failed because network is unavailable in this sandbox. A lockfile was generated with offline-compatible cached dependencies, after which the normal acceptance commands passed.

Commit attempt:

- `git add README.md CODEX_REPORT.md Cargo.toml Cargo.lock assets crates examples schemas tools && git commit -m "Build Harborworks Rust foundation"`: failed because the sandbox reported `.git/index.lock` as read-only.

## Next Recommended Tasks

- Generate schemas directly from Rust types or add full JSON Schema validation in the CLI.
- Expand material and part catalogs with tests for cross-reference coverage.
- Add runtime proxy fields for buoyancy samples, drag samples, propulsor points, compartments, and system graphs.
- Add golden JSON round-trip tests for deterministic blueprint save ordering.
- Start a minimal editor/import path only after the headless contracts settle.
