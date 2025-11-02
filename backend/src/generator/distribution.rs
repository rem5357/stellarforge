use rand::Rng;
use crate::models::star_system::Position3D;
use std::f64::consts::PI;

/// Generate a random position within a cube
pub fn generate_cube_position(size_x: f64, size_y: f64, size_z: f64) -> Position3D {
    let mut rng = rand::thread_rng();

    Position3D {
        x: rng.gen_range(-size_x/2.0..size_x/2.0),
        y: rng.gen_range(-size_y/2.0..size_y/2.0),
        z: rng.gen_range(-size_z/2.0..size_z/2.0),
    }
}

/// Generate a random position within a sphere using polar coordinates
/// This ensures uniform distribution within the sphere volume
pub fn generate_sphere_position(radius: f64) -> Position3D {
    let mut rng = rand::thread_rng();

    // Random distance from center (cube root for uniform volume distribution)
    let r = rng.gen::<f64>().powf(1.0/3.0) * radius;

    // Random azimuth angle (0 to 2π)
    let theta = rng.gen::<f64>() * 2.0 * PI;

    // Random polar angle (0 to π)
    // Using acos for uniform distribution on sphere surface
    let phi = (rng.gen::<f64>() * 2.0 - 1.0).acos();

    // Convert spherical coordinates to Cartesian
    Position3D {
        x: r * phi.sin() * theta.cos(),
        y: r * phi.sin() * theta.sin(),
        z: r * phi.cos(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_position() {
        let pos = generate_cube_position(100.0, 100.0, 100.0);
        assert!(pos.x >= -50.0 && pos.x <= 50.0);
        assert!(pos.y >= -50.0 && pos.y <= 50.0);
        assert!(pos.z >= -50.0 && pos.z <= 50.0);
    }

    #[test]
    fn test_sphere_position() {
        let radius = 100.0;
        let pos = generate_sphere_position(radius);
        let distance = (pos.x.powi(2) + pos.y.powi(2) + pos.z.powi(2)).sqrt();
        assert!(distance <= radius, "Point should be within sphere radius");
    }

    #[test]
    fn test_sphere_distribution() {
        // Test that points are distributed throughout the volume
        let radius = 100.0;
        let mut min_dist = f64::MAX;
        let mut max_dist = 0.0;

        for _ in 0..1000 {
            let pos = generate_sphere_position(radius);
            let dist = (pos.x.powi(2) + pos.y.powi(2) + pos.z.powi(2)).sqrt();
            min_dist = min_dist.min(dist);
            max_dist = max_dist.max(dist);
        }

        // Should have points near center and near edge
        assert!(min_dist < radius * 0.2, "Should have points near center");
        assert!(max_dist > radius * 0.8, "Should have points near edge");
    }
}
