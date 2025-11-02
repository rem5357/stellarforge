use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystem {
    pub id: Option<Uuid>,
    pub project_id: Uuid,
    pub name: String,
    pub system_type: SystemType,
    pub x_ly: f64,
    pub y_ly: f64,
    pub z_ly: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SystemType {
    Solo,
    Binary,
    Trinary,
}

impl SystemType {
    pub fn to_string(&self) -> String {
        match self {
            SystemType::Solo => "solo".to_string(),
            SystemType::Binary => "binary".to_string(),
            SystemType::Trinary => "trinary".to_string(),
        }
    }

    pub fn num_stars(&self) -> usize {
        match self {
            SystemType::Solo => 1,
            SystemType::Binary => 2,
            SystemType::Trinary => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Position3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
