use uuid::Uuid;
use rand::Rng;
use crate::models::{StarSystem, Star, SystemType};
use super::{
    generate_cube_position, generate_sphere_position,
    generate_system_name, generate_star_names,
    generate_star_properties,
};

/// Determine system type based on weighted random
/// Solo: 74%, Binary: 25%, Trinary: 1%
pub fn determine_system_type() -> SystemType {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(0..100);

    if roll < 74 {
        SystemType::Solo
    } else if roll < 99 {
        SystemType::Binary
    } else {
        SystemType::Trinary
    }
}

/// Generate all star systems for a project
pub fn generate_star_systems(
    project_id: Uuid,
    num_systems: usize,
    distribution_type: &str,
    cube_params: Option<(f64, f64, f64)>,
    sphere_params: Option<f64>,
) -> Result<(Vec<StarSystem>, Vec<Star>), String> {
    let mut systems = Vec::new();
    let mut stars = Vec::new();

    for i in 0..num_systems {
        // Generate position
        let position = match distribution_type {
            "cube" => {
                let (x, y, z) = cube_params.ok_or("Cube parameters required")?;
                generate_cube_position(x, y, z)
            },
            "sphere" => {
                let radius = sphere_params.ok_or("Sphere parameters required")?;
                generate_sphere_position(radius)
            },
            _ => return Err("Invalid distribution type".to_string()),
        };

        // Determine system type
        let system_type = determine_system_type();

        // Generate system name
        let system_name = generate_system_name(i);

        // Create star system
        let system = StarSystem {
            id: Some(Uuid::new_v4()),
            project_id,
            name: system_name.clone(),
            system_type: system_type.clone(),
            x_ly: position.x,
            y_ly: position.y,
            z_ly: position.z,
        };

        let system_id = system.id.unwrap();

        // Generate star names for this system
        let star_names = generate_star_names(&system_name, &system_type);

        // Create stars
        for star_name in star_names {
            let properties = generate_star_properties();

            let star = Star {
                id: Some(Uuid::new_v4()),
                system_id,
                project_id,
                name: star_name,
                spectral_class: properties.spectral_class,
                spectral_subclass: Some(properties.spectral_subclass),
                luminosity_class: "V".to_string(), // Main sequence
                temperature_k: properties.temperature_k,
                mass_solar: properties.mass_solar,
                radius_solar: properties.radius_solar,
                luminosity_solar: properties.luminosity_solar,
            };

            stars.push(star);
        }

        systems.push(system);
    }

    Ok((systems, stars))
}

/// Get system type counts from generated systems
pub fn count_system_types(systems: &[StarSystem]) -> (i32, i32, i32) {
    let solo = systems.iter().filter(|s| s.system_type == SystemType::Solo).count() as i32;
    let binary = systems.iter().filter(|s| s.system_type == SystemType::Binary).count() as i32;
    let trinary = systems.iter().filter(|s| s.system_type == SystemType::Trinary).count() as i32;

    (solo, binary, trinary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_type_distribution() {
        let mut solo_count = 0;
        let mut binary_count = 0;
        let mut trinary_count = 0;

        // Generate 10,000 systems
        for _ in 0..10000 {
            match determine_system_type() {
                SystemType::Solo => solo_count += 1,
                SystemType::Binary => binary_count += 1,
                SystemType::Trinary => trinary_count += 1,
            }
        }

        let total = 10000.0;
        let solo_pct = (solo_count as f64 / total) * 100.0;
        let binary_pct = (binary_count as f64 / total) * 100.0;
        let trinary_pct = (trinary_count as f64 / total) * 100.0;

        println!("Solo: {:.1}%, Binary: {:.1}%, Trinary: {:.1}%", solo_pct, binary_pct, trinary_pct);

        // Allow ±5% variance
        assert!(solo_pct >= 69.0 && solo_pct <= 79.0, "Solo systems should be around 74%");
        assert!(binary_pct >= 20.0 && binary_pct <= 30.0, "Binary systems should be around 25%");
        assert!(trinary_pct >= 0.0 && trinary_pct <= 6.0, "Trinary systems should be around 1%");
    }

    #[test]
    fn test_generate_star_systems_sphere() {
        let project_id = Uuid::new_v4();
        let (systems, stars) = generate_star_systems(
            project_id,
            100,
            "sphere",
            None,
            Some(100.0),
        ).unwrap();

        assert_eq!(systems.len(), 100);
        assert!(stars.len() >= 100 && stars.len() <= 300); // Between 1-3 stars per system

        // Check all systems have correct project_id
        for system in &systems {
            assert_eq!(system.project_id, project_id);
        }

        // Check all stars have correct project_id
        for star in &stars {
            assert_eq!(star.project_id, project_id);
        }
    }

    #[test]
    fn test_generate_star_systems_cube() {
        let project_id = Uuid::new_v4();
        let (systems, stars) = generate_star_systems(
            project_id,
            50,
            "cube",
            Some((100.0, 100.0, 100.0)),
            None,
        ).unwrap();

        assert_eq!(systems.len(), 50);
        assert!(stars.len() >= 50 && stars.len() <= 150);

        // Check positions are within cube bounds
        for system in &systems {
            assert!(system.x_ly >= -50.0 && system.x_ly <= 50.0);
            assert!(system.y_ly >= -50.0 && system.y_ly <= 50.0);
            assert!(system.z_ly >= -50.0 && system.z_ly <= 50.0);
        }
    }

    #[test]
    fn test_count_system_types() {
        let project_id = Uuid::new_v4();
        let (systems, _) = generate_star_systems(
            project_id,
            100,
            "sphere",
            None,
            Some(100.0),
        ).unwrap();

        let (solo, binary, trinary) = count_system_types(&systems);

        assert_eq!(solo + binary + trinary, 100);
        assert!(solo >= 60 && solo <= 85); // Roughly 74%
        assert!(binary >= 15 && binary <= 35); // Roughly 25%
        assert!(trinary >= 0 && trinary <= 10); // Roughly 1%
    }
}
