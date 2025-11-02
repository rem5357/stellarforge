-- StellarForge Helper Functions and Views

\c stellarforge
SET search_path TO stellar, public;

-- =============================================================================
-- VIEW: Project Statistics
-- =============================================================================

CREATE OR REPLACE VIEW project_stats AS
SELECT
    p.id,
    p.name,
    p.distribution_type,
    p.num_star_systems,
    p.created_at,
    COUNT(DISTINCT ss.id) AS actual_systems_created,
    COUNT(s.id) AS total_stars,
    SUM(CASE WHEN ss.system_type = 'solo' THEN 1 ELSE 0 END) AS solo_count,
    SUM(CASE WHEN ss.system_type = 'binary' THEN 1 ELSE 0 END) AS binary_count,
    SUM(CASE WHEN ss.system_type = 'trinary' THEN 1 ELSE 0 END) AS trinary_count,
    SUM(CASE WHEN s.spectral_class = 'O' THEN 1 ELSE 0 END) AS o_class_count,
    SUM(CASE WHEN s.spectral_class = 'B' THEN 1 ELSE 0 END) AS b_class_count,
    SUM(CASE WHEN s.spectral_class = 'A' THEN 1 ELSE 0 END) AS a_class_count,
    SUM(CASE WHEN s.spectral_class = 'F' THEN 1 ELSE 0 END) AS f_class_count,
    SUM(CASE WHEN s.spectral_class = 'G' THEN 1 ELSE 0 END) AS g_class_count,
    SUM(CASE WHEN s.spectral_class = 'K' THEN 1 ELSE 0 END) AS k_class_count,
    SUM(CASE WHEN s.spectral_class = 'M' THEN 1 ELSE 0 END) AS m_class_count
FROM projects p
LEFT JOIN star_systems ss ON ss.project_id = p.id
LEFT JOIN stars s ON s.project_id = p.id
GROUP BY p.id, p.name, p.distribution_type, p.num_star_systems, p.created_at;

COMMENT ON VIEW project_stats IS 'Aggregated statistics for each project';

-- =============================================================================
-- FUNCTION: Update Project Statistics
-- Called after generation to update project stats
-- =============================================================================

CREATE OR REPLACE FUNCTION update_project_stats(project_uuid UUID)
RETURNS void AS $$
BEGIN
    UPDATE projects p
    SET
        total_stars = (SELECT COUNT(*) FROM stars WHERE project_id = project_uuid),
        solo_systems = (SELECT COUNT(*) FROM star_systems WHERE project_id = project_uuid AND system_type = 'solo'),
        binary_systems = (SELECT COUNT(*) FROM star_systems WHERE project_id = project_uuid AND system_type = 'binary'),
        trinary_systems = (SELECT COUNT(*) FROM star_systems WHERE project_id = project_uuid AND system_type = 'trinary'),
        updated_at = NOW()
    WHERE id = project_uuid;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION update_project_stats IS 'Update project statistics after star generation';

-- =============================================================================
-- FUNCTION: Get Nearest Star Systems
-- Find N nearest star systems to a given point
-- =============================================================================

CREATE OR REPLACE FUNCTION get_nearest_systems(
    project_uuid UUID,
    x NUMERIC,
    y NUMERIC,
    z NUMERIC,
    limit_count INTEGER DEFAULT 10
)
RETURNS TABLE (
    system_id UUID,
    system_name VARCHAR,
    distance_ly NUMERIC,
    x_ly NUMERIC,
    y_ly NUMERIC,
    z_ly NUMERIC,
    system_type VARCHAR
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        ss.id,
        ss.name,
        SQRT(POWER(ss.x_ly - x, 2) + POWER(ss.y_ly - y, 2) + POWER(ss.z_ly - z, 2)) AS distance,
        ss.x_ly,
        ss.y_ly,
        ss.z_ly,
        ss.system_type
    FROM star_systems ss
    WHERE ss.project_id = project_uuid
    ORDER BY distance
    LIMIT limit_count;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION get_nearest_systems IS 'Find nearest star systems to a point';

-- =============================================================================
-- FUNCTION: Get Systems Within Radius
-- Find all star systems within a given radius from a point
-- =============================================================================

CREATE OR REPLACE FUNCTION get_systems_within_radius(
    project_uuid UUID,
    center_x NUMERIC,
    center_y NUMERIC,
    center_z NUMERIC,
    radius NUMERIC
)
RETURNS TABLE (
    system_id UUID,
    system_name VARCHAR,
    distance_ly NUMERIC,
    x_ly NUMERIC,
    y_ly NUMERIC,
    z_ly NUMERIC,
    system_type VARCHAR,
    num_stars INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        ss.id,
        ss.name,
        SQRT(POWER(ss.x_ly - center_x, 2) + POWER(ss.y_ly - center_y, 2) + POWER(ss.z_ly - center_z, 2)) AS distance,
        ss.x_ly,
        ss.y_ly,
        ss.z_ly,
        ss.system_type,
        (SELECT COUNT(*)::INTEGER FROM stars s WHERE s.system_id = ss.id) AS num_stars
    FROM star_systems ss
    WHERE ss.project_id = project_uuid
      AND SQRT(POWER(ss.x_ly - center_x, 2) + POWER(ss.y_ly - center_y, 2) + POWER(ss.z_ly - center_z, 2)) <= radius
    ORDER BY distance;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION get_systems_within_radius IS 'Find all systems within a spherical radius';

-- =============================================================================
-- FUNCTION: Get Spectral Class Distribution
-- Returns count of stars by spectral class for a project
-- =============================================================================

CREATE OR REPLACE FUNCTION get_spectral_distribution(project_uuid UUID)
RETURNS TABLE (
    spectral_class CHAR(1),
    count BIGINT,
    percentage NUMERIC
) AS $$
BEGIN
    RETURN QUERY
    WITH class_counts AS (
        SELECT
            s.spectral_class,
            COUNT(*) AS cnt
        FROM stars s
        WHERE s.project_id = project_uuid
        GROUP BY s.spectral_class
    ),
    total AS (
        SELECT SUM(cnt) AS total_count FROM class_counts
    )
    SELECT
        cc.spectral_class,
        cc.cnt,
        ROUND((cc.cnt::NUMERIC / t.total_count::NUMERIC) * 100, 2) AS pct
    FROM class_counts cc, total t
    ORDER BY
        CASE cc.spectral_class
            WHEN 'O' THEN 1
            WHEN 'B' THEN 2
            WHEN 'A' THEN 3
            WHEN 'F' THEN 4
            WHEN 'G' THEN 5
            WHEN 'K' THEN 6
            WHEN 'M' THEN 7
        END;
END;
$$ LANGUAGE plpgsql;

COMMENT ON FUNCTION get_spectral_distribution IS 'Get star count and percentage by spectral class';

-- Success message
SELECT 'Helper functions and views created successfully!' AS status;
