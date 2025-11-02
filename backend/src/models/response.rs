use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::{Project, Star, StarSystem};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateProjectResponse {
    pub project_id: Uuid,
    pub name: String,
    pub num_star_systems: i32,
    pub num_stars_total: i32,
    pub solo_systems: i32,
    pub binary_systems: i32,
    pub trinary_systems: i32,
    pub generation_time_ms: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectListResponse {
    pub projects: Vec<ProjectSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub id: Uuid,
    pub name: String,
    pub num_star_systems: i32,
    pub num_stars_total: i32,
    pub created_at: String,
    pub distribution_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStarsResponse {
    pub project_id: Uuid,
    pub project_name: String,
    pub systems: Vec<SystemWithStars>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemWithStars {
    pub system_name: String,
    pub system_type: String,
    pub position: Position,
    pub stars: Vec<StarDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarDetail {
    pub name: String,
    pub spectral_class: String,
    pub temperature_k: f64,
    pub mass_solar: f64,
    pub radius_solar: f64,
    pub luminosity_solar: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
