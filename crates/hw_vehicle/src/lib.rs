//! Editable vehicle blueprints and data validation.
//!
//! Responsibilities:
//! - Code-only blueprint model for nodes, beams, panels, component placeholders, routes, and controls.
//! - Serde save/load with deterministic normalization helpers.
//! - Topology and asset validation for the Milestone 0 foundation.
//!
//! Non-responsibilities:
//! - Runtime physics, graphical editing, mesh generation, full CAD behavior, or real hydrodynamics.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use hw_core::{
    BLUEPRINT_SCHEMA_VERSION, BeamId, CompartmentId, ComponentId, Diagnostic, MaterialId, NodeId,
    PanelId, PartId, PortId, RouteId, VEHICLE_CLASS_MARINE_SMALL, ValidationReport,
};
use hw_math::{Transform, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlueprintMeta {
    pub blueprint_id: String,
    pub display_name: String,
    pub author: String,
    pub version: String,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VehicleBlueprint {
    pub schema_version: u32,
    pub vehicle_class: String,
    pub meta: BlueprintMeta,
    #[serde(default)]
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub beams: Vec<Beam>,
    #[serde(default)]
    pub panels: Vec<Panel>,
    #[serde(default)]
    pub components: Vec<ComponentInstance>,
    #[serde(default)]
    pub routes: Vec<Route>,
    #[serde(default)]
    pub control_links: Vec<ControlLink>,
    #[serde(default)]
    pub compartments: Vec<Compartment>,
}

impl VehicleBlueprint {
    pub fn load_json(path: impl AsRef<Path>) -> Result<Self, BlueprintIoError> {
        let text = fs::read_to_string(path)?;
        let mut blueprint: Self = serde_json::from_str(&text)?;
        blueprint.normalize();
        Ok(blueprint)
    }

    pub fn to_pretty_json(&self) -> Result<String, serde_json::Error> {
        let mut normalized = self.clone();
        normalized.normalize();
        serde_json::to_string_pretty(&normalized)
    }

    pub fn normalize(&mut self) {
        self.nodes.sort_by(|a, b| a.id.cmp(&b.id));
        self.beams.sort_by(|a, b| a.id.cmp(&b.id));
        self.panels.sort_by(|a, b| a.id.cmp(&b.id));
        self.components.sort_by(|a, b| a.id.cmp(&b.id));
        for component in &mut self.components {
            component.ports.sort_by(|a, b| a.id.cmp(&b.id));
        }
        self.routes.sort_by(|a, b| a.id.cmp(&b.id));
        self.control_links.sort_by(|a, b| a.id.cmp(&b.id));
        self.compartments.sort_by(|a, b| a.id.cmp(&b.id));
    }

    pub fn validate(&self) -> ValidationReport {
        let mut report = ValidationReport::new();

        if self.schema_version != BLUEPRINT_SCHEMA_VERSION {
            report.error(
                "blueprint.schema_version",
                format!(
                    "unsupported schema_version {}; expected {}",
                    self.schema_version, BLUEPRINT_SCHEMA_VERSION
                ),
            );
        }
        if self.vehicle_class != VEHICLE_CLASS_MARINE_SMALL {
            report.error(
                "blueprint.vehicle_class",
                format!(
                    "unsupported vehicle_class {}; expected {}",
                    self.vehicle_class, VEHICLE_CLASS_MARINE_SMALL
                ),
            );
        }

        let node_ids = collect_unique(&mut report, "node", self.nodes.iter().map(|n| &n.id));
        let beam_ids = collect_unique(&mut report, "beam", self.beams.iter().map(|b| &b.id));
        let panel_ids = collect_unique(&mut report, "panel", self.panels.iter().map(|p| &p.id));
        let component_ids = collect_unique(
            &mut report,
            "component",
            self.components.iter().map(|c| &c.id),
        );
        let route_ids = collect_unique(&mut report, "route", self.routes.iter().map(|r| &r.id));
        let compartment_ids = collect_unique(
            &mut report,
            "compartment",
            self.compartments.iter().map(|c| &c.id),
        );
        drop((beam_ids, panel_ids, route_ids, compartment_ids));

        let mut ports: BTreeMap<PortId, ComponentId> = BTreeMap::new();
        for component in &self.components {
            if !component.transform.is_finite() {
                report.error(
                    "component.transform.finite",
                    format!("component {} has a non-finite transform", component.id),
                );
            }
            if component.part_id.is_empty() {
                report.error(
                    "component.part_id.empty",
                    format!("component {} has an empty part_id", component.id),
                );
            }
            if component.mass_kg < 0.0 || !component.mass_kg.is_finite() {
                report.error(
                    "component.mass.non_negative",
                    format!(
                        "component {} has invalid mass_kg {}",
                        component.id, component.mass_kg
                    ),
                );
            }
            let _ = collect_unique(
                &mut report,
                "port",
                component.ports.iter().map(|port| &port.id),
            );
            for port in &component.ports {
                if ports
                    .insert(port.id.clone(), component.id.clone())
                    .is_some()
                {
                    report.error("port.duplicate", format!("duplicate port id {}", port.id));
                }
                if !port.local_position.is_finite() {
                    report.error(
                        "port.position.finite",
                        format!("port {} has a non-finite local_position", port.id),
                    );
                }
            }
        }

        for node in &self.nodes {
            if !node.position.is_finite() {
                report.error(
                    "node.position.finite",
                    format!("node {} has a non-finite position", node.id),
                );
            }
        }

        for beam in &self.beams {
            if !node_ids.contains(&beam.node_a) {
                report.error(
                    "beam.node_a.exists",
                    format!("beam {} references missing node_a {}", beam.id, beam.node_a),
                );
            }
            if !node_ids.contains(&beam.node_b) {
                report.error(
                    "beam.node_b.exists",
                    format!("beam {} references missing node_b {}", beam.id, beam.node_b),
                );
            }
            if beam.node_a == beam.node_b {
                report.error(
                    "beam.nodes.distinct",
                    format!("beam {} connects node {} to itself", beam.id, beam.node_a),
                );
            }
            if beam.wall_thickness_m < 0.0 || !beam.wall_thickness_m.is_finite() {
                report.error(
                    "beam.wall_thickness.non_negative",
                    format!(
                        "beam {} has invalid wall_thickness_m {}",
                        beam.id, beam.wall_thickness_m
                    ),
                );
            }
        }

        for panel in &self.panels {
            if panel.boundary_nodes.len() < 3 {
                report.error(
                    "panel.boundary.minimum",
                    format!("panel {} has fewer than 3 boundary nodes", panel.id),
                );
            }
            if panel.thickness_m < 0.0 || !panel.thickness_m.is_finite() {
                report.error(
                    "panel.thickness.non_negative",
                    format!(
                        "panel {} has invalid thickness_m {}",
                        panel.id, panel.thickness_m
                    ),
                );
            }
            for node_id in &panel.boundary_nodes {
                if !node_ids.contains(node_id) {
                    report.error(
                        "panel.boundary_node.exists",
                        format!(
                            "panel {} references missing boundary node {}",
                            panel.id, node_id
                        ),
                    );
                }
            }
        }

        for route in &self.routes {
            if route.length_m < 0.0 || !route.length_m.is_finite() {
                report.error(
                    "route.length.non_negative",
                    format!("route {} has invalid length_m {}", route.id, route.length_m),
                );
            }
            for endpoint in [&route.from, &route.to] {
                if !component_ids.contains(&endpoint.component_id) {
                    report.error(
                        "route.component.exists",
                        format!(
                            "route {} references missing component {}",
                            route.id, endpoint.component_id
                        ),
                    );
                }
                match ports.get(&endpoint.port_id) {
                    Some(owner) if owner == &endpoint.component_id => {}
                    Some(owner) => report.error(
                        "route.port.owner",
                        format!(
                            "route {} port {} belongs to component {}, not {}",
                            route.id, endpoint.port_id, owner, endpoint.component_id
                        ),
                    ),
                    None => report.error(
                        "route.port.exists",
                        format!(
                            "route {} references missing port {}",
                            route.id, endpoint.port_id
                        ),
                    ),
                }
            }
        }

        for link in &self.control_links {
            if !component_ids.contains(&link.source_component_id) {
                report.error(
                    "control.source.exists",
                    format!(
                        "control link {} references missing source component {}",
                        link.id, link.source_component_id
                    ),
                );
            }
            if !component_ids.contains(&link.target_component_id) {
                report.error(
                    "control.target.exists",
                    format!(
                        "control link {} references missing target component {}",
                        link.id, link.target_component_id
                    ),
                );
            }
        }

        report
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Node {
    pub id: NodeId,
    pub position: Vec3,
    #[serde(default)]
    pub flags: Vec<String>,
    #[serde(default)]
    pub snap_group: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Beam {
    pub id: BeamId,
    pub node_a: NodeId,
    pub node_b: NodeId,
    pub profile_id: PartId,
    pub material_id: MaterialId,
    pub wall_thickness_m: f32,
    #[serde(default)]
    pub connection_type: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Panel {
    pub id: PanelId,
    pub boundary_nodes: Vec<NodeId>,
    pub material_id: MaterialId,
    pub thickness_m: f32,
    #[serde(default)]
    pub seal_rating: f32,
    #[serde(default)]
    pub compartment_boundary: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComponentInstance {
    pub id: ComponentId,
    pub part_id: PartId,
    pub transform: Transform,
    #[serde(default)]
    pub mass_kg: f32,
    #[serde(default)]
    pub ports: Vec<Port>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Port {
    pub id: PortId,
    pub kind: PortKind,
    pub local_position: Vec3,
    #[serde(default)]
    pub capacity: f32,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortKind {
    ElectricIn,
    ElectricOut,
    SignalIn,
    SignalOut,
    FuelIn,
    FuelOut,
    WaterIn,
    WaterOut,
    ShaftIn,
    ShaftOut,
    MechanicalControlIn,
    MechanicalControlOut,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Route {
    pub id: RouteId,
    pub route_type: RouteType,
    pub from: RouteEndpoint,
    pub to: RouteEndpoint,
    #[serde(default)]
    pub length_m: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RouteEndpoint {
    pub component_id: ComponentId,
    pub port_id: PortId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteType {
    Wire,
    FuelLine,
    WaterHose,
    Shaft,
    ControlCable,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ControlLink {
    pub id: RouteId,
    pub source_component_id: ComponentId,
    pub target_component_id: ComponentId,
    #[serde(default)]
    pub signal: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Compartment {
    pub id: CompartmentId,
    pub marker: Vec3,
    pub volume_m3: f32,
    #[serde(default)]
    pub is_watertight: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MaterialDefinition {
    pub schema_version: u32,
    pub material_id: MaterialId,
    pub display_name: String,
    pub density_kg_m3: f32,
    pub yield_strength_mpa: f32,
    pub impact_toughness: f32,
    pub corrosion_resistance: f32,
    pub repair_difficulty: f32,
    pub cost_per_kg: f32,
    pub watertight_rating: f32,
}

impl MaterialDefinition {
    pub fn load_json(path: impl AsRef<Path>) -> Result<Self, BlueprintIoError> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn validate(&self) -> ValidationReport {
        let mut report = ValidationReport::new();
        if self.schema_version != 1 {
            report.error(
                "material.schema_version",
                format!(
                    "material {} has unsupported schema_version",
                    self.material_id
                ),
            );
        }
        validate_non_negative(
            &mut report,
            "material.density",
            self.density_kg_m3,
            "density_kg_m3",
        );
        validate_non_negative(
            &mut report,
            "material.yield_strength",
            self.yield_strength_mpa,
            "yield_strength_mpa",
        );
        validate_non_negative(
            &mut report,
            "material.impact_toughness",
            self.impact_toughness,
            "impact_toughness",
        );
        validate_non_negative(
            &mut report,
            "material.corrosion_resistance",
            self.corrosion_resistance,
            "corrosion_resistance",
        );
        validate_non_negative(
            &mut report,
            "material.repair_difficulty",
            self.repair_difficulty,
            "repair_difficulty",
        );
        validate_non_negative(
            &mut report,
            "material.cost_per_kg",
            self.cost_per_kg,
            "cost_per_kg",
        );
        validate_non_negative(
            &mut report,
            "material.watertight_rating",
            self.watertight_rating,
            "watertight_rating",
        );
        report
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BeamPartDefinition {
    pub schema_version: u32,
    pub part_id: PartId,
    pub display_name: String,
    pub category: String,
    pub outer_diameter_m: f32,
    pub wall_thickness_m: f32,
    pub cost_each: f32,
}

impl BeamPartDefinition {
    pub fn load_json(path: impl AsRef<Path>) -> Result<Self, BlueprintIoError> {
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn validate(&self) -> ValidationReport {
        let mut report = ValidationReport::new();
        if self.schema_version != 1 {
            report.error(
                "part.schema_version",
                format!("part {} has unsupported schema_version", self.part_id),
            );
        }
        validate_non_negative(
            &mut report,
            "part.outer_diameter",
            self.outer_diameter_m,
            "outer_diameter_m",
        );
        validate_non_negative(
            &mut report,
            "part.wall_thickness",
            self.wall_thickness_m,
            "wall_thickness_m",
        );
        validate_non_negative(&mut report, "part.cost_each", self.cost_each, "cost_each");
        report
    }

    pub fn tube_area_m2(&self) -> f32 {
        let outer_radius = self.outer_diameter_m * 0.5;
        let inner_radius = (outer_radius - self.wall_thickness_m).max(0.0);
        std::f32::consts::PI * (outer_radius.powi(2) - inner_radius.powi(2))
    }
}

fn collect_unique<'a, T>(
    report: &mut ValidationReport,
    label: &'static str,
    ids: impl Iterator<Item = &'a T>,
) -> BTreeSet<T>
where
    T: Clone + Ord + std::fmt::Display + 'a,
{
    let mut seen = BTreeSet::new();
    for id in ids {
        let cloned = id.clone();
        if format!("{cloned}").trim().is_empty() {
            report.error("id.empty", format!("{label} id is empty"));
        }
        if !seen.insert(cloned.clone()) {
            report.error("id.duplicate", format!("duplicate {label} id {cloned}"));
        }
    }
    seen
}

fn validate_non_negative(
    report: &mut ValidationReport,
    code: &'static str,
    value: f32,
    field: &str,
) {
    if value < 0.0 || !value.is_finite() {
        report.error(code, format!("{field} must be finite and non-negative"));
    }
}

#[derive(Debug)]
pub enum BlueprintIoError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for BlueprintIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::Json(error) => write!(f, "JSON error: {error}"),
        }
    }
}

impl std::error::Error for BlueprintIoError {}

impl From<std::io::Error> for BlueprintIoError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for BlueprintIoError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

pub fn diagnostics_to_string(diagnostics: &[Diagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diagnostic| {
            format!(
                "{:?} {}: {}",
                diagnostic.severity, diagnostic.code, diagnostic.message
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_load_does_not_lose_ids() {
        let blueprint = sample_blueprint();
        let json = blueprint.to_pretty_json().unwrap();
        let loaded: VehicleBlueprint = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.nodes[0].id, NodeId::from("n1"));
        assert_eq!(loaded.beams[0].id, BeamId::from("b1"));
    }

    #[test]
    fn invalid_beam_endpoint_rejected_clearly() {
        let mut blueprint = sample_blueprint();
        blueprint.beams[0].node_b = NodeId::from("missing");
        let report = blueprint.validate();
        assert!(!report.is_ok());
        assert!(
            diagnostics_to_string(report.diagnostics())
                .contains("beam b1 references missing node_b missing")
        );
    }

    fn sample_blueprint() -> VehicleBlueprint {
        VehicleBlueprint {
            schema_version: 1,
            vehicle_class: "marine_small".to_string(),
            meta: BlueprintMeta {
                blueprint_id: "sample".to_string(),
                display_name: "Sample".to_string(),
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
                    position: Vec3::new(1.0, 0.0, 0.0),
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
            components: vec![],
            routes: vec![],
            control_links: vec![],
            compartments: vec![],
        }
    }
}
