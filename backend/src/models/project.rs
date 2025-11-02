use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // Generation parameters
    pub distribution_type: String,
    pub num_star_systems: i32,

    // Cube parameters
    pub size_x_ly: Option<f64>,
    pub size_y_ly: Option<f64>,
    pub size_z_ly: Option<f64>,

    // Sphere parameters
    pub radius_ly: Option<f64>,

    // Statistics
    pub total_stars: i32,
    pub solo_systems: i32,
    pub binary_systems: i32,
    pub trinary_systems: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributionType {
    Cube { size_x: f64, size_y: f64, size_z: f64 },
    Sphere { radius: f64 },
}

impl DistributionType {
    pub fn to_string(&self) -> String {
        match self {
            DistributionType::Cube { .. } => "cube".to_string(),
            DistributionType::Sphere { .. } => "sphere".to_string(),
        }
    }
}
