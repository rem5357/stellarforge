use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use crate::models::star::{SpectralClass, StarProperties};

/// Generate random spectral class with weighted distribution
/// M stars are 20x more common than O stars
pub fn generate_spectral_class() -> SpectralClass {
    let mut rng = rand::thread_rng();

    let classes = SpectralClass::all_classes();
    let weights: Vec<u32> = classes.iter().map(|c| c.weight()).collect();

    let dist = WeightedIndex::new(&weights).unwrap();
    classes[dist.sample(&mut rng)]
}

/// Generate complete star properties based on spectral class
pub fn generate_star_properties() -> StarProperties {
    let mut rng = rand::thread_rng();

    let spectral_class = generate_spectral_class();
    let spectral_subclass = rng.gen_range(0..=9);

    let (base_temp, base_mass, base_radius, base_lum) = spectral_class.typical_properties();

    // Add some variation (±10%) to make stars within same class unique
    let variation = || 1.0 + rng.gen_range(-0.1..0.1);

    StarProperties {
        spectral_class,
        spectral_subclass,
        temperature_k: base_temp * variation(),
        mass_solar: base_mass * variation(),
        radius_solar: base_radius * variation(),
        luminosity_solar: base_lum * variation(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_class_distribution() {
        let mut counts = std::collections::HashMap::new();

        // Generate 10,000 stars
        for _ in 0..10000 {
            let class = generate_spectral_class();
            *counts.entry(class).or_insert(0) += 1;
        }

        // M stars should be roughly 20x more common than O stars
        let m_count = counts.get(&SpectralClass::M).unwrap_or(&0);
        let o_count = counts.get(&SpectralClass::O).unwrap_or(&0);

        println!("M stars: {}, O stars: {}, ratio: {:.1}", m_count, o_count, *m_count as f64 / *o_count as f64);

        // Allow some variance (15x to 25x)
        assert!(*m_count > *o_count * 15, "M stars should be much more common");
    }

    #[test]
    fn test_star_properties() {
        let props = generate_star_properties();

        // Temperature should be in valid range
        assert!(props.temperature_k >= 2400.0 && props.temperature_k <= 50000.0);

        // Mass should be positive
        assert!(props.mass_solar > 0.0);

        // Subclass should be 0-9
        assert!(props.spectral_subclass >= 0 && props.spectral_subclass <= 9);
    }
}
