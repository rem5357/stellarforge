# StellarForge Database Setup Script
# Run this script to initialize the PostgreSQL database

$ErrorActionPreference = "Stop"

Write-Host "=== StellarForge Database Setup ===" -ForegroundColor Cyan
Write-Host ""

# Configuration
$PGHOST = "localhost"
$PGPORT = "5432"
$PGUSER = "postgres"
$PGPASSWORD = "Beta5357"
$PGDATABASE = "stellarforge"

# Check if PostgreSQL is installed
Write-Host "Checking PostgreSQL installation..." -ForegroundColor Yellow
try {
    $pgVersion = & psql --version 2>&1
    Write-Host "Found: $pgVersion" -ForegroundColor Green
} catch {
    Write-Host "ERROR: PostgreSQL not found. Please install PostgreSQL 14+ first." -ForegroundColor Red
    exit 1
}

# Check if PostGIS is available
Write-Host ""
Write-Host "Checking for PostGIS extension..." -ForegroundColor Yellow
$env:PGPASSWORD = $PGPASSWORD
$postgisCheck = & psql -h $PGHOST -p $PGPORT -U $PGUSER -d postgres -t -c "SELECT 1 FROM pg_available_extensions WHERE name = 'postgis';" 2>&1
if ($postgisCheck -match "1") {
    Write-Host "PostGIS extension is available" -ForegroundColor Green
} else {
    Write-Host "WARNING: PostGIS extension not found. Please install PostGIS." -ForegroundColor Yellow
    Write-Host "Download from: https://postgis.net/windows_downloads/" -ForegroundColor Yellow
}

# Drop existing database if it exists (optional, commented out for safety)
Write-Host ""
Write-Host "Checking if database '$PGDATABASE' exists..." -ForegroundColor Yellow
$dbExists = & psql -h $PGHOST -p $PGPORT -U $PGUSER -d postgres -t -c "SELECT 1 FROM pg_database WHERE datname = '$PGDATABASE';" 2>&1
if ($dbExists -match "1") {
    Write-Host "Database '$PGDATABASE' already exists." -ForegroundColor Yellow
    $response = Read-Host "Do you want to drop and recreate it? (yes/no)"
    if ($response -eq "yes") {
        Write-Host "Dropping existing database..." -ForegroundColor Yellow
        & psql -h $PGHOST -p $PGPORT -U $PGUSER -d postgres -c "DROP DATABASE IF EXISTS $PGDATABASE;" 2>&1 | Out-Null
        Write-Host "Database dropped." -ForegroundColor Green
    } else {
        Write-Host "Skipping database creation. Proceeding with existing database." -ForegroundColor Yellow
    }
}

# Create database if it doesn't exist
if ($response -eq "yes" -or -not ($dbExists -match "1")) {
    Write-Host ""
    Write-Host "Creating database '$PGDATABASE'..." -ForegroundColor Yellow
    & psql -h $PGHOST -p $PGPORT -U $PGUSER -d postgres -c "CREATE DATABASE $PGDATABASE;" 2>&1 | Out-Null
    Write-Host "Database created successfully!" -ForegroundColor Green
}

# Run SQL scripts in order
Write-Host ""
Write-Host "Running SQL migration scripts..." -ForegroundColor Yellow
Write-Host ""

$sqlFiles = @(
    "sql/01_create_database.sql",
    "sql/02_create_tables.sql",
    "sql/03_helper_functions.sql"
)

foreach ($sqlFile in $sqlFiles) {
    if (Test-Path $sqlFile) {
        Write-Host "Executing: $sqlFile" -ForegroundColor Cyan
        $env:PGPASSWORD = $PGPASSWORD
        & psql -h $PGHOST -p $PGPORT -U $PGUSER -d $PGDATABASE -f $sqlFile 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  ✓ Success" -ForegroundColor Green
        } else {
            Write-Host "  ✗ Failed" -ForegroundColor Red
            Write-Host "  Check the error above for details" -ForegroundColor Red
            exit 1
        }
    } else {
        Write-Host "  ✗ File not found: $sqlFile" -ForegroundColor Red
        exit 1
    }
}

# Verify setup
Write-Host ""
Write-Host "Verifying database setup..." -ForegroundColor Yellow
$env:PGPASSWORD = $PGPASSWORD
$tableCount = & psql -h $PGHOST -p $PGPORT -U $PGUSER -d $PGDATABASE -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'stellar';" 2>&1
Write-Host "Found $tableCount tables in 'stellar' schema" -ForegroundColor Green

# Display connection info
Write-Host ""
Write-Host "=== Database Setup Complete! ===" -ForegroundColor Green
Write-Host ""
Write-Host "Connection Details:" -ForegroundColor Cyan
Write-Host "  Host:     $PGHOST" -ForegroundColor White
Write-Host "  Port:     $PGPORT" -ForegroundColor White
Write-Host "  Database: $PGDATABASE" -ForegroundColor White
Write-Host "  User:     $PGUSER" -ForegroundColor White
Write-Host "  Password: $PGPASSWORD" -ForegroundColor White
Write-Host ""
Write-Host "Connection String:" -ForegroundColor Cyan
Write-Host "  postgresql://${PGUSER}:${PGPASSWORD}@${PGHOST}:${PGPORT}/${PGDATABASE}" -ForegroundColor White
Write-Host ""
Write-Host "Next Steps:" -ForegroundColor Cyan
Write-Host "  1. Create the Rust backend: cd backend && cargo build" -ForegroundColor White
Write-Host "  2. Create the Blazor frontend: cd blazor && dotnet build" -ForegroundColor White
Write-Host "  3. Configure Nginx reverse proxy" -ForegroundColor White
Write-Host ""
