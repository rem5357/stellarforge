use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;
use std::time::Instant;

use crate::database::{DbPool, Repository};
use crate::generator::generate_star_systems;
use crate::models::{
    GenerateProjectRequest, GenerateProjectResponse, ProjectListResponse,
    ProjectSummary, ProjectStarsResponse, SystemWithStars, Position,
    StarDetail, ErrorResponse,
};

/// POST /api/projects/generate
/// Generate a new project with star systems
#[post("/projects/generate")]
async fn generate_project(
    pool: web::Data<DbPool>,
    req: web::Json<GenerateProjectRequest>,
) -> impl Responder {
    let start_time = Instant::now();

    // Validate request
    if let Err(e) = req.validate() {
        return HttpResponse::BadRequest().json(ErrorResponse { error: e });
    }

    let repo = Repository::new(pool.get_ref().clone());

    // Create project
    let project_id = match repo
        .create_project(
            &req.name,
            None,
            &req.distribution_type,
            req.num_stars,
            if req.distribution_type == "cube" {
                Some((
                    req.size_x_ly.unwrap(),
                    req.size_y_ly.unwrap(),
                    req.size_z_ly.unwrap(),
                ))
            } else {
                None
            },
            req.radius_ly,
        )
        .await
    {
        Ok(id) => id,
        Err(e) => {
            log::error!("Failed to create project: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Database error: {}", e),
            });
        }
    };

    // Generate star systems
    let (systems, stars) = match generate_star_systems(
        project_id,
        req.num_stars as usize,
        &req.distribution_type,
        if req.distribution_type == "cube" {
            Some((
                req.size_x_ly.unwrap(),
                req.size_y_ly.unwrap(),
                req.size_z_ly.unwrap(),
            ))
        } else {
            None
        },
        req.radius_ly,
    ) {
        Ok(result) => result,
        Err(e) => {
            log::error!("Failed to generate star systems: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Generation error: {}", e),
            });
        }
    };

    // Count system types
    let (solo_count, binary_count, trinary_count) = crate::generator::count_system_types(&systems);

    // Insert into database
    if let Err(e) = repo.insert_star_systems(&systems).await {
        log::error!("Failed to insert star systems: {}", e);
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Database error: {}", e),
        });
    }

    if let Err(e) = repo.insert_stars(&stars).await {
        log::error!("Failed to insert stars: {}", e);
        return HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("Database error: {}", e),
        });
    }

    // Update project statistics
    if let Err(e) = repo.update_project_stats(project_id).await {
        log::error!("Failed to update project stats: {}", e);
    }

    let generation_time_ms = start_time.elapsed().as_millis();

    log::info!(
        "Generated project '{}': {} systems, {} stars in {}ms",
        req.name,
        systems.len(),
        stars.len(),
        generation_time_ms
    );

    HttpResponse::Ok().json(GenerateProjectResponse {
        project_id,
        name: req.name.clone(),
        num_star_systems: systems.len() as i32,
        num_stars_total: stars.len() as i32,
        solo_systems: solo_count,
        binary_systems: binary_count,
        trinary_systems: trinary_count,
        generation_time_ms,
    })
}

/// GET /api/projects
/// List all projects
#[get("/projects")]
async fn list_projects(pool: web::Data<DbPool>) -> impl Responder {
    let repo = Repository::new(pool.get_ref().clone());

    let projects = match repo.get_all_projects().await {
        Ok(p) => p,
        Err(e) => {
            log::error!("Failed to get projects: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Database error: {}", e),
            });
        }
    };

    let summaries: Vec<ProjectSummary> = projects
        .into_iter()
        .map(|p| ProjectSummary {
            id: p.id,
            name: p.name,
            num_star_systems: p.num_star_systems,
            num_stars_total: p.total_stars,
            created_at: p.created_at.to_rfc3339(),
            distribution_type: p.distribution_type,
        })
        .collect();

    HttpResponse::Ok().json(ProjectListResponse {
        projects: summaries,
    })
}

/// GET /api/projects/{id}
/// Get project details
#[get("/projects/{id}")]
async fn get_project(pool: web::Data<DbPool>, project_id: web::Path<Uuid>) -> impl Responder {
    let repo = Repository::new(pool.get_ref().clone());

    let project = match repo.get_project_by_id(*project_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                error: "Project not found".to_string(),
            });
        }
        Err(e) => {
            log::error!("Failed to get project: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Database error: {}", e),
            });
        }
    };

    HttpResponse::Ok().json(project)
}

/// GET /api/projects/{id}/stars
/// Get all stars for a project
#[get("/projects/{id}/stars")]
async fn get_project_stars(
    pool: web::Data<DbPool>,
    project_id: web::Path<Uuid>,
) -> impl Responder {
    let repo = Repository::new(pool.get_ref().clone());

    // Get project
    let project = match repo.get_project_by_id(*project_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                error: "Project not found".to_string(),
            });
        }
        Err(e) => {
            log::error!("Failed to get project: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Database error: {}", e),
            });
        }
    };

    // Get star systems
    let systems = match repo.get_star_systems_by_project(*project_id).await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to get star systems: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Database error: {}", e),
            });
        }
    };

    // Get stars
    let stars = match repo.get_stars_by_project(*project_id).await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to get stars: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("Database error: {}", e),
            });
        }
    };

    // Group stars by system
    let mut systems_with_stars: Vec<SystemWithStars> = Vec::new();

    for system in systems {
        let system_stars: Vec<StarDetail> = stars
            .iter()
            .filter(|s| s.system_id == system.id.unwrap())
            .map(|s| StarDetail {
                name: s.name.clone(),
                spectral_class: format!("{}{}", s.spectral_class.to_char(), s.spectral_subclass.unwrap_or(0)),
                temperature_k: s.temperature_k,
                mass_solar: s.mass_solar,
                radius_solar: s.radius_solar,
                luminosity_solar: s.luminosity_solar,
            })
            .collect();

        systems_with_stars.push(SystemWithStars {
            system_name: system.name.clone(),
            system_type: system.system_type.to_string(),
            position: Position {
                x: system.x_ly,
                y: system.y_ly,
                z: system.z_ly,
            },
            stars: system_stars,
        });
    }

    HttpResponse::Ok().json(ProjectStarsResponse {
        project_id: *project_id,
        project_name: project.name,
        systems: systems_with_stars,
    })
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(generate_project)
        .service(list_projects)
        .service(get_project)
        .service(get_project_stars);
}
