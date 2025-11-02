use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    pub id: Option<Uuid>,
    pub system_id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub spectral_class: SpectralClass,
    pub spectral_subclass: Option<i32>,
    pub luminosity_class: String,
    pub temperature_k: f64,
    pub mass_solar: f64,
    pub radius_solar: f64,
    pub luminosity_solar: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpectralClass {
    O, // Blue, very hot
    B, // Blue-white
    A, // White
    F, // Yellow-white
    G, // Yellow (like Sun)
    K, // Orange
    M, // Red, cool
}

impl SpectralClass {
    pub fn to_char(&self) -> char {
        match self {
            SpectralClass::O => 'O',
            SpectralClass::B => 'B',
            SpectralClass::A => 'A',
            SpectralClass::F => 'F',
            SpectralClass::G => 'G',
            SpectralClass::K => 'K',
            SpectralClass::M => 'M',
        }
    }

    /// Returns typical properties for this spectral class
    /// (temperature, mass, radius, luminosity)
    pub fn typical_properties(&self) -> (f64, f64, f64, f64) {
        match self {
            // (temp_k, mass_solar, radius_solar, luminosity_solar)
            SpectralClass::O => (40000.0, 40.0, 15.0, 100000.0),
            SpectralClass::B => (20000.0, 10.0, 7.0, 1000.0),
            SpectralClass::A => (8500.0, 2.5, 2.5, 40.0),
            SpectralClass::F => (6750.0, 1.5, 1.5, 6.0),
            SpectralClass::G => (5500.0, 1.0, 1.0, 1.0), // Sun-like
            SpectralClass::K => (4500.0, 0.7, 0.8, 0.4),
            SpectralClass::M => (3000.0, 0.3, 0.4, 0.04),
        }
    }

    /// Get relative weight for random generation
    /// M stars are 20x more common than O stars
    pub fn weight(&self) -> u32 {
        match self {
            SpectralClass::O => 1,
            SpectralClass::B => 2,
            SpectralClass::A => 4,
            SpectralClass::F => 7,
            SpectralClass::G => 11,
            SpectralClass::K => 15,
            SpectralClass::M => 20,
        }
    }

    pub fn all_classes() -> [SpectralClass; 7] {
        [
            SpectralClass::O,
            SpectralClass::B,
            SpectralClass::A,
            SpectralClass::F,
            SpectralClass::G,
            SpectralClass::K,
            SpectralClass::M,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarProperties {
    pub spectral_class: SpectralClass,
    pub spectral_subclass: i32,
    pub temperature_k: f64,
    pub mass_solar: f64,
    pub radius_solar: f64,
    pub luminosity_solar: f64,
}
