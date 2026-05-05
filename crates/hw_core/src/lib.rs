//! Core Harborworks types shared across crates.
//!
//! Responsibilities:
//! - Strong typed identifiers for editable and runtime graph entities.
//! - Common diagnostics and validation report containers.
//! - Small schema/version constants that should not depend on gameplay crates.
//!
//! Non-responsibilities:
//! - Vehicle topology rules, simulation, rendering, asset loading, or UI behavior.

use std::fmt;

use serde::{Deserialize, Serialize};

pub const BLUEPRINT_SCHEMA_VERSION: u32 = 1;
pub const VEHICLE_CLASS_MARINE_SMALL: &str = "marine_small";

macro_rules! typed_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn is_empty(&self) -> bool {
                self.0.trim().is_empty()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }
    };
}

typed_id!(NodeId);
typed_id!(BeamId);
typed_id!(PanelId);
typed_id!(ComponentId);
typed_id!(PortId);
typed_id!(RouteId);
typed_id!(CompartmentId);
typed_id!(MaterialId);
typed_id!(PartId);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub code: &'static str,
    pub message: String,
}

impl Diagnostic {
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            code,
            message: message.into(),
        }
    }

    pub fn warning(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: DiagnosticSeverity::Warning,
            code,
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidationReport {
    diagnostics: Vec<Diagnostic>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn error(&mut self, code: &'static str, message: impl Into<String>) {
        self.push(Diagnostic::error(code, message));
    }

    pub fn warning(&mut self, code: &'static str, message: impl Into<String>) {
        self.push(Diagnostic::warning(code, message));
    }

    pub fn is_ok(&self) -> bool {
        !self
            .diagnostics
            .iter()
            .any(|d| d.severity == DiagnosticSeverity::Error)
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn extend(&mut self, other: ValidationReport) {
        self.diagnostics.extend(other.diagnostics);
    }
}
