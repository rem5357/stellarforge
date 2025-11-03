# StellarForge

A modern stellar cartography platform for procedurally generating and visualizing star systems.

## Overview

StellarForge is a Blazor WebAssembly application with a Rust backend and PostgreSQL database for creating, managing, and visualizing procedurally generated galaxies. Phase 1 focuses on star system generation with scientifically-inspired distributions.

## Technology Stack

- **Frontend**: Blazor WebAssembly (.NET 8)
- **Backend**: Rust + Actix-Web
- **Database**: PostgreSQL 18 + PostGIS
- **Reverse Proxy**: Nginx
- **Random Generation**: Rust `rand` crate

## Features (Phase 1)

- **Procedural Star Generation**
  - Cube or sphere distribution
  - Weighted spectral class distribution (cool stars 20x more common than hot stars)
  - Binary (25%) and trinary (1%) star systems
  - Sequential naming: S0001, S0001a/b, S0001a/b/c

- **Database Storage**
  - Multiple project/save support
  - PostGIS 3D spatial indexing
  - Fast spatial queries

- **Web Interface**
  - Interactive generation form
  - Real-time results display
  - Project management

## Quick Start

**See [DEPLOYMENT.md](DEPLOYMENT.md) for complete deployment instructions.**

### Prerequisites

- PostgreSQL 18+ with PostGIS
- Rust 1.70+
- .NET 9 SDK
- Nginx (for production)

### 1. Setup Database

**Manual setup required** (PowerShell script has syntax issues - use manual commands):

```powershell
# Windows - Run in PowerShell
$env:PGPASSWORD = "Beta5357"
& "C:\Program Files\PostgreSQL\18\bin\psql.exe" -U postgres -c "CREATE DATABASE stellarforge;"
& "C:\Program Files\PostgreSQL\18\bin\psql.exe" -U postgres -d stellarforge -f D:\projects\stellarforge\sql\01_create_database.sql
& "C:\Program Files\PostgreSQL\18\bin\psql.exe" -U postgres -d stellarforge -f D:\projects\stellarforge\sql\02_create_tables.sql
& "C:\Program Files\PostgreSQL\18\bin\psql.exe" -U postgres -d stellarforge -f D:\projects\stellarforge\sql\03_helper_functions.sql
```

See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed instructions and troubleshooting.

### 2. Start Backend

```bash
cd backend
cargo run --release
```

Backend starts on **http://localhost:8080**

### 3. Deploy to Nginx (Production)

```bash
# Copy nginx.conf to C:\nginx\conf\nginx.conf
# Start nginx
cd C:\nginx
.\nginx.exe
```

Access at: **https://127.0.0.1/stellarforge**

### 4. Or Run Frontend in Dev Mode

```bash
cd blazor/StellarForge.Web
dotnet watch run
```

Access at: **http://localhost:5000** (or displayed URL)

## Project Structure

```
stellarforge/
├── sql/                    # Database schema and migrations
├── backend/                # Rust backend API
│   ├── src/
│   │   ├── api/           # API endpoints
│   │   ├── models/        # Data models
│   │   ├── database/      # Database layer
│   │   └── generator/     # Star generation engine
│   └── Cargo.toml
├── blazor/                 # Blazor WASM frontend (coming soon)
├── docs/                   # Documentation
│   ├── PROJECT.md         # Complete project history
│   ├── PHASE1_ARCHITECTURE.md  # Phase 1 design
│   └── PROGRESS.md        # Development progress
└── README.md
```

## API Endpoints

- `POST /api/projects/generate` - Generate new star project
- `GET /api/projects` - List all projects
- `GET /api/projects/{id}/stars` - Get stars for a project

See [PHASE1_ARCHITECTURE.md](PHASE1_ARCHITECTURE.md) for complete API documentation.

## Star Generation Details

### Spectral Class Distribution

Stars are distributed by spectral class with weights matching astronomical observations:

| Class | Temperature | Color | Weight | Relative Frequency |
|-------|-------------|-------|--------|-------------------|
| O | 30,000-50,000K | Blue | 1 | Rare |
| B | 10,000-30,000K | Blue-white | 2 | Uncommon |
| A | 7,500-10,000K | White | 4 | Uncommon |
| F | 6,000-7,500K | Yellow-white | 7 | Common |
| G | 5,200-6,000K | Yellow | 11 | Common |
| K | 3,700-5,200K | Orange | 15 | Very Common |
| M | 2,400-3,700K | Red | 20 | Most Common |

### System Types

- **Solo Systems** (74%): Single star (e.g., "S0001")
- **Binary Systems** (25%): Two stars (e.g., "S0001a", "S0001b")
- **Trinary Systems** (1%): Three stars (e.g., "S0001a", "S0001b", "S0001c")

### Distribution Types

**Cube**: Uniform random distribution within a rectangular volume
- Parameters: size_x, size_y, size_z (in light-years)
- Stars placed randomly within bounds

**Sphere**: Uniform random distribution within a spherical volume
- Parameters: radius (in light-years)
- Uses polar coordinates with proper volume distribution

## Development

### Running Tests

```bash
cd backend
cargo test
```

### Building for Production

```bash
cd backend
cargo build --release
```

## Documentation

- [PROJECT.md](PROJECT.md) - Complete project history from SolarViewer
- [PHASE1_ARCHITECTURE.md](PHASE1_ARCHITECTURE.md) - Detailed Phase 1 design
- [PROGRESS.md](PROGRESS.md) - Development progress tracking

## Future Phases

- **Phase 2**: 3D visualization, planet generation
- **Phase 3**: Political territories, trade routes
- **Phase 4**: AI-assisted generation and optimization

## License

MIT License - See LICENSE file for details

## Contributing

This project is part of an evolutionary development from SolarViewer, a tool for visualizing Astrosynthesis stellar cartography data.

## Credits

Built with Claude Code as an evolution of the SolarViewer project.

---

**Status**: 🎉 Phase 1 COMPLETE! 🎉
**Repository**: https://github.com/rem5357/stellarforge
**Live URL**: https://127.0.0.1/stellarforge (after deployment)
