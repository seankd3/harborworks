//! Headless deterministic simulation stubs.
//!
//! Responsibilities:
//! - Provide replayable, deterministic telemetry for compiled runtime vehicle proxies.
//! - Exercise command-line and test paths before real physics and systems land.
//!
//! Non-responsibilities:
//! - Real hydrodynamics, real collision, real weather/waves, or player-facing rendering.

use hw_math::Vec3;
use hw_vehicle_compile::RuntimeVehicle;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaySummary {
    pub seconds: f32,
    pub ticks: u32,
    pub final_position: Vec3,
    pub final_speed_mps: f32,
    pub energy_placeholder: f32,
}

pub fn run_replay(vehicle: &RuntimeVehicle, seconds: f32, fixed_dt: f32) -> ReplaySummary {
    let seconds = seconds.max(0.0);
    let fixed_dt = fixed_dt.max(0.001);
    let ticks = (seconds / fixed_dt).round() as u32;
    let thrust_placeholder_n = 1200.0 + vehicle.component_count as f32 * 15.0;
    let drag_factor = 0.18 + vehicle.mass_kg * 0.00002;
    let mut position_z = 0.0_f32;
    let mut speed = 0.0_f32;
    let mut energy = 0.0_f32;

    for _ in 0..ticks {
        let acceleration = thrust_placeholder_n / vehicle.mass_kg - drag_factor * speed;
        speed = (speed + acceleration * fixed_dt).max(0.0);
        position_z += speed * fixed_dt;
        energy += thrust_placeholder_n * speed * fixed_dt;
    }

    ReplaySummary {
        seconds,
        ticks,
        final_position: Vec3::new(0.0, 0.0, position_z),
        final_speed_mps: speed,
        energy_placeholder: energy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sim_replay_deterministic_under_fixed_input() {
        let vehicle = RuntimeVehicle {
            source_blueprint_id: "test".to_string(),
            mass_kg: 250.0,
            center_of_mass: Vec3::ZERO,
            inertia_placeholder: Vec3::new(62.5, 62.5, 62.5),
            beam_count: 1,
            panel_count: 0,
            component_count: 1,
            warnings: vec![],
        };
        let a = run_replay(&vehicle, 10.0, 0.1);
        let b = run_replay(&vehicle, 10.0, 0.1);
        assert_eq!(a, b);
        assert_eq!(a.ticks, 100);
    }
}
