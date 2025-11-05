-- PostgreSQL 18+ with PostGIS

-- Create database (run as postgres superuser)
-- CREATE DATABASE stellarforge;

-- Connect to the database
\c stellarforge

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS postgis;
-- gen_random_uuid() is provided by pgcrypto; keep uuid-ossp optional for compatibility
CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create schema for stellar data
CREATE SCHEMA IF NOT EXISTS stellar;

-- Set search path
SET search_path TO stellar, public;

-- Success message
SELECT 'StellarForge database and extensions created successfully!' AS status;
