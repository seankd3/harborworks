//! Harborworks command-line tools.
//!
//! Responsibilities:
//! - Headless validation of schemas, assets, and example blueprints.
//! - Deterministic replay smoke tests for runtime vehicle scaffolding.
//!
//! Non-responsibilities:
//! - Graphical editing, rendering, mission gameplay, or real physics.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use hw_sim::run_replay;
use hw_vehicle::{BeamPartDefinition, MaterialDefinition, VehicleBlueprint, diagnostics_to_string};
use hw_vehicle_compile::{CompileAssets, compile_blueprint, diagnostics_to_lines};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("validate") => validate_command(),
        Some("sim-replay") => {
            let mut blueprint_path = None;
            let mut seconds = 10.0_f32;
            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--blueprint" => blueprint_path = args.next().map(PathBuf::from),
                    "--seconds" => {
                        let value = args.next().context("--seconds requires a value")?;
                        seconds = value.parse().context("--seconds must be a number")?;
                    }
                    other => bail!("unknown sim-replay argument {other}"),
                }
            }
            let blueprint_path =
                blueprint_path.context("sim-replay requires --blueprint <path>")?;
            sim_replay_command(&blueprint_path, seconds)
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("  cargo run -p hw_tools -- validate");
            eprintln!(
                "  cargo run -p hw_tools -- sim-replay --blueprint <path> --seconds <seconds>"
            );
            bail!("unknown or missing command")
        }
    }
}

fn validate_command() -> Result<()> {
    println!("Harborworks validation");
    validate_json_file("schemas/blueprint_v1.schema.json")?;
    validate_json_file("schemas/material_v1.schema.json")?;
    validate_json_file("schemas/part_v1.schema.json")?;

    let assets = load_assets()?;
    for material in assets.materials.values() {
        let report = material.validate();
        if !report.is_ok() {
            bail!(
                "material {} failed validation:\n{}",
                material.material_id,
                diagnostics_to_string(report.diagnostics())
            );
        }
        println!("ok material {}", material.material_id);
    }
    for part in assets.beam_parts.values() {
        let report = part.validate();
        if !report.is_ok() {
            bail!(
                "part {} failed validation:\n{}",
                part.part_id,
                diagnostics_to_string(report.diagnostics())
            );
        }
        println!("ok part {}", part.part_id);
    }

    let blueprint = VehicleBlueprint::load_json("examples/blueprints/starter_rescue_skiff.json")?;
    let report = blueprint.validate();
    if !report.is_ok() {
        bail!(
            "starter blueprint failed validation:\n{}",
            diagnostics_to_string(report.diagnostics())
        );
    }
    let compiled = compile_blueprint(&blueprint, &assets);
    if !compiled.report.is_ok() {
        bail!(
            "starter blueprint failed compile:\n{}",
            diagnostics_to_lines(&compiled.report).join("\n")
        );
    }
    let vehicle = compiled
        .vehicle
        .context("compiler produced no runtime vehicle")?;
    println!(
        "ok blueprint starter_rescue_skiff mass_kg={:.3} center_of_mass=({:.3},{:.3},{:.3})",
        vehicle.mass_kg,
        vehicle.center_of_mass.x,
        vehicle.center_of_mass.y,
        vehicle.center_of_mass.z
    );
    println!("validation passed");
    Ok(())
}

fn sim_replay_command(blueprint_path: &Path, seconds: f32) -> Result<()> {
    let assets = load_assets()?;
    let blueprint = VehicleBlueprint::load_json(blueprint_path)
        .with_context(|| format!("failed to load blueprint {}", blueprint_path.display()))?;
    let compiled = compile_blueprint(&blueprint, &assets);
    if !compiled.report.is_ok() {
        bail!(
            "blueprint failed compile:\n{}",
            diagnostics_to_lines(&compiled.report).join("\n")
        );
    }
    let vehicle = compiled
        .vehicle
        .context("compiler produced no runtime vehicle")?;
    let summary = run_replay(&vehicle, seconds, 0.1);
    println!("Harborworks deterministic replay");
    println!("blueprint_id={}", vehicle.source_blueprint_id);
    println!("mass_kg={:.3}", vehicle.mass_kg);
    println!("seconds={:.3}", summary.seconds);
    println!("ticks={}", summary.ticks);
    println!(
        "final_position_m=({:.3},{:.3},{:.3})",
        summary.final_position.x, summary.final_position.y, summary.final_position.z
    );
    println!("final_speed_mps={:.3}", summary.final_speed_mps);
    println!("energy_placeholder_j={:.3}", summary.energy_placeholder);
    Ok(())
}

fn validate_json_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let text =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let _: serde_json::Value = serde_json::from_str(&text)
        .with_context(|| format!("failed to parse JSON {}", path.display()))?;
    println!("ok json {}", path.display());
    Ok(())
}

fn load_assets() -> Result<CompileAssets> {
    let mut assets = CompileAssets::default();
    for path in [
        "assets/materials/marine_aluminum_5083.json",
        "assets/materials/mild_steel.json",
    ] {
        let material = MaterialDefinition::load_json(path)
            .with_context(|| format!("failed to load material {path}"))?;
        assets.insert_material(material);
    }
    let part = BeamPartDefinition::load_json("assets/parts/tube_beam_small.json")
        .context("failed to load tube_beam_small")?;
    assets.insert_beam_part(part);
    Ok(assets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starter_blueprint_validates() {
        let blueprint =
            VehicleBlueprint::load_json("../../examples/blueprints/starter_rescue_skiff.json")
                .unwrap_or_else(|_| {
                    VehicleBlueprint::load_json("examples/blueprints/starter_rescue_skiff.json")
                        .unwrap()
                });
        let report = blueprint.validate();
        assert!(
            report.is_ok(),
            "{}",
            diagnostics_to_string(report.diagnostics())
        );
    }

    #[test]
    fn material_schema_and_data_loads() {
        validate_json_file("../../schemas/material_v1.schema.json")
            .or_else(|_| validate_json_file("schemas/material_v1.schema.json"))
            .unwrap();
        let material =
            MaterialDefinition::load_json("../../assets/materials/marine_aluminum_5083.json")
                .or_else(|_| {
                    MaterialDefinition::load_json("assets/materials/marine_aluminum_5083.json")
                })
                .unwrap();
        assert!(
            material.validate().is_ok(),
            "{}",
            diagnostics_to_string(material.validate().diagnostics())
        );
    }
}
