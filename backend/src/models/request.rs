use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateProjectRequest {
    pub name: String,
    pub num_stars: i32,
    pub distribution_type: String,

    // Cube parameters (required if distribution_type = "cube")
    pub size_x_ly: Option<f64>,
    pub size_y_ly: Option<f64>,
    pub size_z_ly: Option<f64>,

    // Sphere parameters (required if distribution_type = "sphere")
    pub radius_ly: Option<f64>,
}

impl GenerateProjectRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Project name cannot be empty".to_string());
        }

        if self.num_stars < 1 || self.num_stars > 10000 {
            return Err("Number of stars must be between 1 and 10,000".to_string());
        }

        match self.distribution_type.as_str() {
            "cube" => {
                if self.size_x_ly.is_none() || self.size_y_ly.is_none() || self.size_z_ly.is_none() {
                    return Err("Cube distribution requires size_x_ly, size_y_ly, and size_z_ly".to_string());
                }
                if self.size_x_ly.unwrap() <= 0.0 || self.size_y_ly.unwrap() <= 0.0 || self.size_z_ly.unwrap() <= 0.0 {
                    return Err("Cube dimensions must be positive".to_string());
                }
            },
            "sphere" => {
                if self.radius_ly.is_none() {
                    return Err("Sphere distribution requires radius_ly".to_string());
                }
                if self.radius_ly.unwrap() <= 0.0 {
                    return Err("Sphere radius must be positive".to_string());
                }
            },
            _ => {
                return Err("distribution_type must be 'cube' or 'sphere'".to_string());
            }
        }

        Ok(())
    }
}
