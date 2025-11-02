-- StellarForge Phase 1 Tables
-- Projects, Star Systems, and Stars

\c stellarforge
SET search_path TO stellar, public;

-- =============================================================================
-- PROJECTS TABLE
-- Stores generation sessions/save games
-- =============================================================================

CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),

    -- Generation parameters
    distribution_type VARCHAR(20) NOT NULL CHECK (distribution_type IN ('cube', 'sphere')),
    num_star_systems INTEGER NOT NULL CHECK (num_star_systems > 0 AND num_star_systems <= 10000),

    -- Cube parameters (NULL if sphere)
    size_x_ly NUMERIC CHECK (size_x_ly > 0),
    size_y_ly NUMERIC CHECK (size_y_ly > 0),
    size_z_ly NUMERIC CHECK (size_z_ly > 0),

    -- Sphere parameters (NULL if cube)
    radius_ly NUMERIC CHECK (radius_ly > 0),

    -- Statistics (updated after generation)
    total_stars INTEGER DEFAULT 0,
    solo_systems INTEGER DEFAULT 0,
    binary_systems INTEGER DEFAULT 0,
    trinary_systems INTEGER DEFAULT 0,

    -- Ensure either cube OR sphere params are set
    CONSTRAINT valid_distribution CHECK (
        (distribution_type = 'cube' AND size_x_ly IS NOT NULL AND size_y_ly IS NOT NULL AND size_z_ly IS NOT NULL AND radius_ly IS NULL) OR
        (distribution_type = 'sphere' AND radius_ly IS NOT NULL AND size_x_ly IS NULL AND size_y_ly IS NULL AND size_z_ly IS NULL)
    )
);

COMMENT ON TABLE projects IS 'Star generation projects/save sessions';
COMMENT ON COLUMN projects.distribution_type IS 'cube = random XYZ within bounds, sphere = random polar coordinates';
COMMENT ON COLUMN projects.num_star_systems IS 'Number of star systems to generate (not individual stars)';

-- =============================================================================
-- STAR SYSTEMS TABLE
-- Each entry is one star system (may contain 1-3 stars)
-- =============================================================================

CREATE TABLE IF NOT EXISTS star_systems (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(20) NOT NULL, -- S0001, S0002, etc.
    system_type VARCHAR(20) NOT NULL CHECK (system_type IN ('solo', 'binary', 'trinary')),

    -- 3D position in light-years
    position GEOMETRY(PointZ, 4326) NOT NULL, -- PostGIS 3D point
    x_ly NUMERIC NOT NULL,
    y_ly NUMERIC NOT NULL,
    z_ly NUMERIC NOT NULL,

    created_at TIMESTAMP DEFAULT NOW(),

    UNIQUE(project_id, name)
);

COMMENT ON TABLE star_systems IS 'Star systems (containers that hold 1-3 stars)';
COMMENT ON COLUMN star_systems.name IS 'System name in format S#### (e.g., S0001, S0002)';
COMMENT ON COLUMN star_systems.system_type IS 'solo = 1 star, binary = 2 stars, trinary = 3 stars';
COMMENT ON COLUMN star_systems.position IS 'PostGIS 3D point for spatial queries';

-- =============================================================================
-- STARS TABLE
-- Individual stars (1 per solo system, 2 per binary, 3 per trinary)
-- =============================================================================

CREATE TABLE IF NOT EXISTS stars (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    system_id UUID NOT NULL REFERENCES star_systems(id) ON DELETE CASCADE,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,

    name VARCHAR(20) NOT NULL, -- S0001, S0001a, S0001b, etc.
    spectral_class CHAR(1) NOT NULL CHECK (spectral_class IN ('O', 'B', 'A', 'F', 'G', 'K', 'M')),
    spectral_subclass INTEGER CHECK (spectral_subclass >= 0 AND spectral_subclass <= 9),
    luminosity_class VARCHAR(5) DEFAULT 'V', -- V = main sequence

    -- Physical properties
    temperature_k NUMERIC NOT NULL CHECK (temperature_k >= 2400 AND temperature_k <= 50000),
    mass_solar NUMERIC NOT NULL CHECK (mass_solar > 0),
    radius_solar NUMERIC NOT NULL CHECK (radius_solar > 0),
    luminosity_solar NUMERIC NOT NULL CHECK (luminosity_solar > 0),

    created_at TIMESTAMP DEFAULT NOW(),

    UNIQUE(project_id, name)
);

COMMENT ON TABLE stars IS 'Individual stars within star systems';
COMMENT ON COLUMN stars.name IS 'Star name: S0001 for solo, S0001a/S0001b for binary/trinary';
COMMENT ON COLUMN stars.spectral_class IS 'O, B, A, F, G, K, M (hot to cool)';
COMMENT ON COLUMN stars.spectral_subclass IS '0-9 subclass (optional, 0 = hottest of class)';
COMMENT ON COLUMN stars.luminosity_class IS 'V = main sequence (dwarf), III = giant, I = supergiant, etc.';
COMMENT ON COLUMN stars.temperature_k IS 'Surface temperature in Kelvin';
COMMENT ON COLUMN stars.mass_solar IS 'Mass in solar masses (1.0 = Sun)';
COMMENT ON COLUMN stars.radius_solar IS 'Radius in solar radii (1.0 = Sun)';
COMMENT ON COLUMN stars.luminosity_solar IS 'Luminosity in solar luminosities (1.0 = Sun)';

-- =============================================================================
-- INDEXES
-- =============================================================================

-- Projects
CREATE INDEX idx_projects_created ON projects(created_at DESC);
CREATE INDEX idx_projects_name ON projects(name);

-- Star Systems
CREATE INDEX idx_star_systems_project ON star_systems(project_id);
CREATE INDEX idx_star_systems_name ON star_systems(name);
CREATE INDEX idx_star_systems_position ON star_systems USING GIST(position);
CREATE INDEX idx_star_systems_type ON star_systems(system_type);

-- Stars
CREATE INDEX idx_stars_system ON stars(system_id);
CREATE INDEX idx_stars_project ON stars(project_id);
CREATE INDEX idx_stars_spectral ON stars(spectral_class);
CREATE INDEX idx_stars_name ON stars(name);

-- =============================================================================
-- TRIGGERS
-- =============================================================================

-- Update project updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER projects_updated_at
    BEFORE UPDATE ON projects
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

-- Success message
SELECT 'StellarForge Phase 1 tables created successfully!' AS status;
SELECT 'Tables: projects, star_systems, stars' AS created_tables;
