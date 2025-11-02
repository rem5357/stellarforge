use uuid::Uuid;
use tokio_postgres::Row;
use crate::database::DbPool;
use crate::models::{Project, StarSystem, Star, SpectralClass};
use anyhow::{Result, Context};

/// Repository for database operations
pub struct Repository {
    pool: DbPool,
}

impl Repository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // =========================================================================
    // PROJECT OPERATIONS
    // =========================================================================

    /// Create a new project
    pub async fn create_project(
        &self,
        name: &str,
        description: Option<&str>,
        distribution_type: &str,
        num_star_systems: i32,
        cube_params: Option<(f64, f64, f64)>,
        sphere_params: Option<f64>,
    ) -> Result<Uuid> {
        let client = self.pool.get().await?;

        let size_x = cube_params.map(|(x, _, _)| x);
        let size_y = cube_params.map(|(_, y, _)| y);
        let size_z = cube_params.map(|(_, _, z)| z);
        let radius = sphere_params;

        let row = client
            .query_one(
                "INSERT INTO stellar.projects
                 (name, description, distribution_type, num_star_systems,
                  size_x_ly, size_y_ly, size_z_ly, radius_ly)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                 RETURNING id",
                &[
                    &name,
                    &description,
                    &distribution_type,
                    &num_star_systems,
                    &size_x,
                    &size_y,
                    &size_z,
                    &radius,
                ],
            )
            .await
            .context("Failed to insert project")?;

        Ok(row.get(0))
    }

    /// Get all projects
    pub async fn get_all_projects(&self) -> Result<Vec<Project>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, name, description, created_at, updated_at,
                        distribution_type, num_star_systems,
                        size_x_ly, size_y_ly, size_z_ly, radius_ly,
                        total_stars, solo_systems, binary_systems, trinary_systems
                 FROM stellar.projects
                 ORDER BY created_at DESC",
                &[],
            )
            .await?;

        Ok(rows.into_iter().map(|row| self.row_to_project(&row)).collect())
    }

    /// Get project by ID
    pub async fn get_project_by_id(&self, project_id: Uuid) -> Result<Option<Project>> {
        let client = self.pool.get().await?;

        let row = client
            .query_opt(
                "SELECT id, name, description, created_at, updated_at,
                        distribution_type, num_star_systems,
                        size_x_ly, size_y_ly, size_z_ly, radius_ly,
                        total_stars, solo_systems, binary_systems, trinary_systems
                 FROM stellar.projects
                 WHERE id = $1",
                &[&project_id],
            )
            .await?;

        Ok(row.map(|r| self.row_to_project(&r)))
    }

    /// Update project statistics
    pub async fn update_project_stats(&self, project_id: Uuid) -> Result<()> {
        let client = self.pool.get().await?;

        client
            .execute(
                "SELECT stellar.update_project_stats($1)",
                &[&project_id],
            )
            .await?;

        Ok(())
    }

    // =========================================================================
    // STAR SYSTEM OPERATIONS
    // =========================================================================

    /// Insert star systems in batch
    pub async fn insert_star_systems(&self, systems: &[StarSystem]) -> Result<Vec<Uuid>> {
        let mut client = self.pool.get().await?;
        let transaction = client.transaction().await?;

        let mut ids = Vec::with_capacity(systems.len());

        for system in systems {
            let row = transaction
                .query_one(
                    "INSERT INTO stellar.star_systems
                     (project_id, name, system_type, position, x_ly, y_ly, z_ly)
                     VALUES ($1, $2, $3, ST_MakePoint($4, $5, $6), $4, $5, $6)
                     RETURNING id",
                    &[
                        &system.project_id,
                        &system.name,
                        &system.system_type.to_string(),
                        &system.x_ly,
                        &system.y_ly,
                        &system.z_ly,
                    ],
                )
                .await?;

            ids.push(row.get(0));
        }

        transaction.commit().await?;
        Ok(ids)
    }

    /// Get star systems for a project
    pub async fn get_star_systems_by_project(&self, project_id: Uuid) -> Result<Vec<StarSystem>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, project_id, name, system_type, x_ly, y_ly, z_ly
                 FROM stellar.star_systems
                 WHERE project_id = $1
                 ORDER BY name",
                &[&project_id],
            )
            .await?;

        Ok(rows.into_iter().map(|row| self.row_to_star_system(&row)).collect())
    }

    // =========================================================================
    // STAR OPERATIONS
    // =========================================================================

    /// Insert stars in batch
    pub async fn insert_stars(&self, stars: &[Star]) -> Result<Vec<Uuid>> {
        let mut client = self.pool.get().await?;
        let transaction = client.transaction().await?;

        let mut ids = Vec::with_capacity(stars.len());

        for star in stars {
            let row = transaction
                .query_one(
                    "INSERT INTO stellar.stars
                     (system_id, project_id, name, spectral_class, spectral_subclass,
                      luminosity_class, temperature_k, mass_solar, radius_solar, luminosity_solar)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                     RETURNING id",
                    &[
                        &star.system_id,
                        &star.project_id,
                        &star.name,
                        &star.spectral_class.to_char().to_string(),
                        &star.spectral_subclass,
                        &star.luminosity_class,
                        &star.temperature_k,
                        &star.mass_solar,
                        &star.radius_solar,
                        &star.luminosity_solar,
                    ],
                )
                .await?;

            ids.push(row.get(0));
        }

        transaction.commit().await?;
        Ok(ids)
    }

    /// Get stars for a project
    pub async fn get_stars_by_project(&self, project_id: Uuid) -> Result<Vec<Star>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT s.id, s.system_id, s.project_id, s.name,
                        s.spectral_class, s.spectral_subclass, s.luminosity_class,
                        s.temperature_k, s.mass_solar, s.radius_solar, s.luminosity_solar
                 FROM stellar.stars s
                 WHERE s.project_id = $1
                 ORDER BY s.name",
                &[&project_id],
            )
            .await?;

        Ok(rows.into_iter().map(|row| self.row_to_star(&row)).collect())
    }

    /// Get stars for a specific star system
    pub async fn get_stars_by_system(&self, system_id: Uuid) -> Result<Vec<Star>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, system_id, project_id, name,
                        spectral_class, spectral_subclass, luminosity_class,
                        temperature_k, mass_solar, radius_solar, luminosity_solar
                 FROM stellar.stars
                 WHERE system_id = $1
                 ORDER BY name",
                &[&system_id],
            )
            .await?;

        Ok(rows.into_iter().map(|row| self.row_to_star(&row)).collect())
    }

    // =========================================================================
    // HELPER METHODS
    // =========================================================================

    fn row_to_project(&self, row: &Row) -> Project {
        Project {
            id: row.get(0),
            name: row.get(1),
            description: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
            distribution_type: row.get(5),
            num_star_systems: row.get(6),
            size_x_ly: row.get(7),
            size_y_ly: row.get(8),
            size_z_ly: row.get(9),
            radius_ly: row.get(10),
            total_stars: row.get(11),
            solo_systems: row.get(12),
            binary_systems: row.get(13),
            trinary_systems: row.get(14),
        }
    }

    fn row_to_star_system(&self, row: &Row) -> StarSystem {
        let system_type_str: String = row.get(3);
        let system_type = match system_type_str.as_str() {
            "solo" => crate::models::SystemType::Solo,
            "binary" => crate::models::SystemType::Binary,
            "trinary" => crate::models::SystemType::Trinary,
            _ => crate::models::SystemType::Solo,
        };

        StarSystem {
            id: Some(row.get(0)),
            project_id: row.get(1),
            name: row.get(2),
            system_type,
            x_ly: row.get(4),
            y_ly: row.get(5),
            z_ly: row.get(6),
        }
    }

    fn row_to_star(&self, row: &Row) -> Star {
        let spectral_str: String = row.get(4);
        let spectral_class = match spectral_str.chars().next().unwrap_or('M') {
            'O' => SpectralClass::O,
            'B' => SpectralClass::B,
            'A' => SpectralClass::A,
            'F' => SpectralClass::F,
            'G' => SpectralClass::G,
            'K' => SpectralClass::K,
            'M' => SpectralClass::M,
            _ => SpectralClass::M,
        };

        Star {
            id: Some(row.get(0)),
            system_id: row.get(1),
            project_id: row.get(2),
            name: row.get(3),
            spectral_class,
            spectral_subclass: row.get(5),
            luminosity_class: row.get(6),
            temperature_k: row.get(7),
            mass_solar: row.get(8),
            radius_solar: row.get(9),
            luminosity_solar: row.get(10),
        }
    }
}
