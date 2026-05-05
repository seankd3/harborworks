//! Blueprint-to-runtime compiler scaffolding.
//!
//! Responsibilities:
//! - Parse, normalize, and validate editable vehicle topology.
//! - Compute honest placeholder aggregate mass and inertia values from beams/components.
//! - Produce a deterministic `RuntimeVehicle` proxy for headless tests and future systems.
//!
//! Non-responsibilities:
//! - Real CAD geometry, real boat hydrodynamics, finite element analysis, or renderer assets.

use std::collections::BTreeMap;

use hw_core::{Diagnostic, MaterialId, PartId, ValidationReport};
use hw_math::Vec3;
use hw_vehicle::{BeamPartDefinition, MaterialDefinition, VehicleBlueprint};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeVehicle {
    pub source_blueprint_id: String,
    pub mass_kg: f32,
    pub center_of_mass: Vec3,
    pub inertia_placeholder: Vec3,
    pub beam_count: usize,
    pub panel_count: usize,
    pub component_count: usize,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct CompileAssets {
    pub materials: BTreeMap<MaterialId, MaterialDefinition>,
    pub beam_parts: BTreeMap<PartId, BeamPartDefinition>,
}

impl CompileAssets {
    pub fn insert_material(&mut self, material: MaterialDefinition) {
        self.materials
            .insert(material.material_id.clone(), material);
    }

    pub fn insert_beam_part(&mut self, part: BeamPartDefinition) {
        self.beam_parts.insert(part.part_id.clone(), part);
    }
}

#[derive(Clone, Debug)]
pub struct CompileOutput {
    pub vehicle: Option<RuntimeVehicle>,
    pub report: ValidationReport,
}

pub fn compile_blueprint(blueprint: &VehicleBlueprint, assets: &CompileAssets) -> CompileOutput {
    let mut normalized = blueprint.clone();
    normalized.normalize();

    let mut report = normalized.validate();
    if !report.is_ok() {
        return CompileOutput {
            vehicle: None,
            report,
        };
    }

    let mut total_mass = 0.0_f32;
    let mut weighted_position = glam::Vec3::ZERO;
    let mut warnings = Vec::new();

    for beam in &normalized.beams {
        let Some(node_a) = normalized.nodes.iter().find(|node| node.id == beam.node_a) else {
            continue;
        };
        let Some(node_b) = normalized.nodes.iter().find(|node| node.id == beam.node_b) else {
            continue;
        };
        let Some(material) = assets.materials.get(&beam.material_id) else {
            report.error(
                "compile.material.exists",
                format!(
                    "beam {} references missing material {}",
                    beam.id, beam.material_id
                ),
            );
            continue;
        };
        let Some(part) = assets.beam_parts.get(&beam.profile_id) else {
            report.error(
                "compile.part.exists",
                format!(
                    "beam {} references missing beam part {}",
                    beam.id, beam.profile_id
                ),
            );
            continue;
        };
        let length = node_a.position.distance(node_b.position);
        if length <= f32::EPSILON {
            warnings.push(format!("beam {} has near-zero length", beam.id));
            continue;
        }
        let mass = length * part.tube_area_m2() * material.density_kg_m3;
        let midpoint = (node_a.position.as_glam() + node_b.position.as_glam()) * 0.5;
        total_mass += mass;
        weighted_position += midpoint * mass;
    }

    for component in &normalized.components {
        if component.mass_kg > 0.0 {
            total_mass += component.mass_kg;
            weighted_position += component.transform.translation.as_glam() * component.mass_kg;
        } else {
            warnings.push(format!(
                "component {} has zero placeholder mass",
                component.id
            ));
        }
    }

    if total_mass <= 0.0 || !total_mass.is_finite() {
        report.error(
            "compile.mass.positive",
            format!("compiled vehicle mass must be finite and positive, got {total_mass}"),
        );
    }

    if !report.is_ok() {
        return CompileOutput {
            vehicle: None,
            report,
        };
    }

    let center_of_mass = Vec3::from(weighted_position / total_mass);
    let inertia_scalar = (total_mass * 0.25).max(1.0);
    let vehicle = RuntimeVehicle {
        source_blueprint_id: normalized.meta.blueprint_id,
        mass_kg: total_mass,
        center_of_mass,
        inertia_placeholder: Vec3::new(inertia_scalar, inertia_scalar, inertia_scalar),
        beam_count: normalized.beams.len(),
        panel_count: normalized.panels.len(),
        component_count: normalized.components.len(),
        warnings,
    };

    CompileOutput {
        vehicle: Some(vehicle),
        report,
    }
}

pub fn diagnostics_to_lines(report: &ValidationReport) -> Vec<String> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| match diagnostic {
            Diagnostic {
                code,
                message,
                severity,
            } => format!("{severity:?} {code}: {message}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hw_core::{BeamId, MaterialId, NodeId, PartId};
    use hw_math::Transform;
    use hw_vehicle::{Beam, BlueprintMeta, ComponentInstance, Node};

    #[test]
    fn compiled_vehicle_has_finite_positive_mass_and_inertia() {
        let blueprint = VehicleBlueprint {
            schema_version: 1,
            vehicle_class: "marine_small".to_string(),
            meta: BlueprintMeta {
                blueprint_id: "compile-test".to_string(),
                display_name: "Compile Test".to_string(),
                author: "Harborworks".to_string(),
                version: "0.1.0".to_string(),
                created_at: "2026-05-05T00:00:00Z".to_string(),
                modified_at: "2026-05-05T00:00:00Z".to_string(),
            },
            nodes: vec![
                Node {
                    id: NodeId::from("n1"),
                    position: Vec3::ZERO,
                    flags: vec![],
                    snap_group: None,
                    label: None,
                },
                Node {
                    id: NodeId::from("n2"),
                    position: Vec3::new(2.0, 0.0, 0.0),
                    flags: vec![],
                    snap_group: None,
                    label: None,
                },
            ],
            beams: vec![Beam {
                id: BeamId::from("b1"),
                node_a: NodeId::from("n1"),
                node_b: NodeId::from("n2"),
                profile_id: PartId::from("tube_beam_small"),
                material_id: MaterialId::from("marine_aluminum_5083"),
                wall_thickness_m: 0.003,
                connection_type: "welded".to_string(),
            }],
            panels: vec![],
            components: vec![ComponentInstance {
                id: "battery_1".into(),
                part_id: "battery_small".into(),
                transform: Transform::default(),
                mass_kg: 18.0,
                ports: vec![],
            }],
            routes: vec![],
            control_links: vec![],
            compartments: vec![],
        };
        let mut assets = CompileAssets::default();
        assets.insert_material(MaterialDefinition {
            schema_version: 1,
            material_id: "marine_aluminum_5083".into(),
            display_name: "Marine Aluminum 5083".to_string(),
            density_kg_m3: 2650.0,
            yield_strength_mpa: 215.0,
            impact_toughness: 0.65,
            corrosion_resistance: 0.9,
            repair_difficulty: 0.45,
            cost_per_kg: 6.5,
            watertight_rating: 0.95,
        });
        assets.insert_beam_part(BeamPartDefinition {
            schema_version: 1,
            part_id: "tube_beam_small".into(),
            display_name: "Small Tube Beam".to_string(),
            category: "beam_profile".to_string(),
            outer_diameter_m: 0.06,
            wall_thickness_m: 0.003,
            cost_each: 18.0,
        });
        let output = compile_blueprint(&blueprint, &assets);
        let vehicle = output.vehicle.expect("vehicle should compile");
        assert!(vehicle.mass_kg.is_finite() && vehicle.mass_kg > 0.0);
        assert!(vehicle.inertia_placeholder.x.is_finite() && vehicle.inertia_placeholder.x > 0.0);
    }
}
