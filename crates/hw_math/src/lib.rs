//! Math and unit primitives for Harborworks.
//!
//! Responsibilities:
//! - Stable serde-friendly vector and transform wrappers.
//! - Small unit helpers for meters, kilograms, seconds, and related quantities.
//!
//! Non-responsibilities:
//! - Geometry generation, physics integration, rendering math, or CAD kernels.

use glam::Vec3 as GlamVec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn distance(self, other: Self) -> f32 {
        (self.as_glam() - other.as_glam()).length()
    }

    pub fn as_glam(self) -> GlamVec3 {
        GlamVec3::new(self.x, self.y, self.z)
    }
}

impl From<GlamVec3> for Vec3 {
    fn from(value: GlamVec3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl From<Vec3> for GlamVec3 {
    fn from(value: Vec3) -> Self {
        value.as_glam()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation_euler_deg: Vec3,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation_euler_deg: Vec3::ZERO,
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Transform {
    pub fn is_finite(self) -> bool {
        self.translation.is_finite()
            && self.rotation_euler_deg.is_finite()
            && self.scale.is_finite()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Meters(pub f32);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Kilograms(pub f32);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Seconds(pub f32);
