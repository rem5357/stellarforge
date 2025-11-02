use crate::models::star_system::SystemType;

/// Generate star system name
/// Format: S0001, S0002, S0003, etc.
pub fn generate_system_name(index: usize) -> String {
    format!("S{:04}", index + 1)
}

/// Generate star names for a system
/// Solo: S0001
/// Binary: S0001a, S0001b
/// Trinary: S0001a, S0001b, S0001c
pub fn generate_star_names(system_name: &str, system_type: &SystemType) -> Vec<String> {
    match system_type {
        SystemType::Solo => vec![system_name.to_string()],
        SystemType::Binary => vec![
            format!("{}a", system_name),
            format!("{}b", system_name),
        ],
        SystemType::Trinary => vec![
            format!("{}a", system_name),
            format!("{}b", system_name),
            format!("{}c", system_name),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_name_generation() {
        assert_eq!(generate_system_name(0), "S0001");
        assert_eq!(generate_system_name(1), "S0002");
        assert_eq!(generate_system_name(99), "S0100");
        assert_eq!(generate_system_name(9999), "S10000");
    }

    #[test]
    fn test_solo_star_names() {
        let names = generate_star_names("S0001", &SystemType::Solo);
        assert_eq!(names.len(), 1);
        assert_eq!(names[0], "S0001");
    }

    #[test]
    fn test_binary_star_names() {
        let names = generate_star_names("S0001", &SystemType::Binary);
        assert_eq!(names.len(), 2);
        assert_eq!(names[0], "S0001a");
        assert_eq!(names[1], "S0001b");
    }

    #[test]
    fn test_trinary_star_names() {
        let names = generate_star_names("S0001", &SystemType::Trinary);
        assert_eq!(names.len(), 3);
        assert_eq!(names[0], "S0001a");
        assert_eq!(names[1], "S0001b");
        assert_eq!(names[2], "S0001c");
    }
}
