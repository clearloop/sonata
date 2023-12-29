//! Manifest of the site.

use serde::Deserialize;

/// Manifest of the site.
#[derive(Debug, Clone, Deserialize)]
pub struct Manifest {
    /// The name of the site.
    pub name: String,
}
