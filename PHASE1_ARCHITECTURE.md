# StellarForge Phase 1: Star Generation System

## Overview

Phase 1 creates a Blazor WASM application with a Rust backend and PostgreSQL database for procedurally generating star systems. Users specify generation parameters through a web UI, and the Rust backend creates stars with scientifically-inspired distributions.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Browser (Client)                          │
│  ┌────────────────────────────────────────────────────┐     │
│  │         Blazor WASM Frontend                       │     │
│  │                                                     │     │
│  │  ┌──────────────────────────────────────────┐     │     │
│  │  │  Star Generation Form                    │     │     │
│  │  │  - Number of stars                       │     │     │
│  │  │  - Distribution type (cube/sphere)       │     │     │
│  │  │  - Dimensions/radius (LY)                │     │     │
│  │  │  - Project save name                     │     │     │
│  │  └──────────────────────────────────────────┘     │     │
│  │                                                     │     │
│  │  ┌──────────────────────────────────────────┐     │     │
│  │  │  Results Display                         │     │     │
│  │  │  - List of generated stars               │     │     │
│  │  │  - Basic statistics                      │     │     │
│  │  └──────────────────────────────────────────┘     │     │
│  └────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
                        ↓ HTTP/HTTPS
┌─────────────────────────────────────────────────────────────┐
│                   Nginx Reverse Proxy                       │
│                   (Port 80/443)                             │
│                                                             │
│  Static Files (/) ──→ Blazor WASM Assets                   │
│  API (/api/*)    ──→ Rust Backend (Port 8080)              │
└─────────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────────┐
│              Rust Backend (Actix-Web)                       │
│              (localhost:8080)                               │
│                                                             │
│  POST /api/projects/generate                                │
│  ├─ Parse request parameters                               │
│  ├─ Generate star systems with random generator            │
│  │  ├─ Temperature distribution (20x cool to hot)          │
│  │  ├─ Binary stars (25% chance)                           │
│  │  ├─ Trinary stars (1% chance)                           │
│  │  └─ Naming: S0001, S0001a, S0001b, etc.                 │
│  └─ Save to PostgreSQL                                      │
│                                                             │
│  GET /api/projects                                          │
│  └─ List all saved projects                                │
│                                                             │
│  GET /api/projects/{id}/stars                              │
│  └─ Retrieve stars for a project                           │
└─────────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────────┐
│         PostgreSQL + PostGIS Database                       │
│         (localhost:5432)                                    │
│                                                             │
│  Tables:                                                    │
│  - projects (save sessions)                                │
│  - star_systems (S0001, S0002, etc.)                       │
│  - stars (individual stars with a, b, c variants)          │
└─────────────────────────────────────────────────────────────┘
```

## Technology Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Frontend | Blazor WebAssembly (.NET 8) | Interactive UI for star generation |
| Backend API | Rust + Actix-Web | REST API, star generation logic |
| Database | PostgreSQL 18 + PostGIS | Store projects and generated stars |
| Reverse Proxy | Nginx | Serve static files, proxy API requests |
| Random Gen | Rust `rand` crate | Star distribution and properties |

## Phase 1 Requirements

### Star Generation Parameters

1. **Number of Stars**: Integer (1-10,000)
2. **Distribution Type**:
   - **Cube**: Random (x, y, z) within bounds
     - Size X (light-years)
     - Size Y (light-years)
     - Size Z (light-years)
   - **Sphere**: Random polar coordinates
     - Radius (light-years)
     - Random distance from center: `0 to radius`
     - Random azimuth angle: `0° to 360°`
     - Random polar angle: `0° to 180°`
3. **Project Name**: String (unique identifier for save)

### Star Generation Rules

#### Temperature/Size Distribution
- **Cool/Small Stars (M-class)**: 20x more likely than hot stars
- **Hot/Large Stars (O-class)**: Least likely
- **Linear distribution** between extremes

Spectral classes (hot to cool):
```
O - Blue, very hot (30,000-50,000 K)    - Weight: 1
B - Blue-white (10,000-30,000 K)        - Weight: 2
A - White (7,500-10,000 K)              - Weight: 4
F - Yellow-white (6,000-7,500 K)        - Weight: 7
G - Yellow (5,200-6,000 K)              - Weight: 11
K - Orange (3,700-5,200 K)              - Weight: 15
M - Red (2,400-3,700 K)                 - Weight: 20
```

#### Star System Composition
- **Solo Star Systems**: 74% chance
  - System name: `S0001`
  - Star name: `S0001` (same as system)
- **Binary Star Systems**: 25% chance
  - System name: `S0001`
  - Star names: `S0001a`, `S0001b`
- **Trinary Star Systems**: 1% chance
  - System name: `S0001`
  - Star names: `S0001a`, `S0001b`, `S0001c`

#### Star Naming
- Sequential numbering: `S0001`, `S0002`, `S0003`, ...
- Zero-padded to 4 digits (supports up to 9,999 systems)
- Multiple stars append lowercase letters: `a`, `b`, `c`

## Database Schema (Phase 1)

```sql
-- Projects table (save sessions)
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT NOW(),
    distribution_type VARCHAR(20) NOT NULL, -- 'cube' or 'sphere'
    num_stars INTEGER NOT NULL,
    -- Cube parameters
    size_x_ly NUMERIC,
    size_y_ly NUMERIC,
    size_z_ly NUMERIC,
    -- Sphere parameters
    radius_ly NUMERIC,
    notes TEXT
);

-- Star systems (containers)
CREATE TABLE star_systems (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(20) NOT NULL, -- S0001, S0002, etc.
    system_type VARCHAR(20) NOT NULL, -- 'solo', 'binary', 'trinary'
    position GEOMETRY(PointZ, 4326) NOT NULL, -- PostGIS 3D point
    x_ly NUMERIC NOT NULL,
    y_ly NUMERIC NOT NULL,
    z_ly NUMERIC NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(project_id, name)
);

-- Individual stars
CREATE TABLE stars (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    system_id UUID NOT NULL REFERENCES star_systems(id) ON DELETE CASCADE,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(20) NOT NULL, -- S0001a, S0001b, etc.
    spectral_class CHAR(1) NOT NULL, -- O, B, A, F, G, K, M
    temperature_k NUMERIC NOT NULL,
    mass_solar NUMERIC NOT NULL,
    radius_solar NUMERIC NOT NULL,
    luminosity_solar NUMERIC NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(project_id, name)
);

-- Indexes for fast queries
CREATE INDEX idx_star_systems_project ON star_systems(project_id);
CREATE INDEX idx_star_systems_position ON star_systems USING GIST(position);
CREATE INDEX idx_stars_system ON stars(system_id);
CREATE INDEX idx_stars_project ON stars(project_id);
CREATE INDEX idx_stars_spectral ON stars(spectral_class);
```

## Rust Backend Structure

```
stellarforge-backend/
├── Cargo.toml
├── src/
│   ├── main.rs                    # Actix-Web server setup
│   ├── api/
│   │   ├── mod.rs
│   │   ├── projects.rs            # Project CRUD endpoints
│   │   └── generation.rs          # Star generation endpoint
│   ├── models/
│   │   ├── mod.rs
│   │   ├── project.rs             # Project struct
│   │   ├── star_system.rs         # StarSystem struct
│   │   └── star.rs                # Star struct
│   ├── database/
│   │   ├── mod.rs
│   │   ├── connection.rs          # PostgreSQL connection pool
│   │   └── repository.rs          # Database operations
│   ├── generator/
│   │   ├── mod.rs
│   │   ├── distribution.rs        # Cube/Sphere distribution logic
│   │   ├── star_properties.rs     # Spectral class, temperature, etc.
│   │   └── naming.rs              # Star naming logic
│   └── config.rs                  # Configuration management
└── .env                           # Environment variables
```

### Key API Endpoints

#### POST /api/projects/generate
Generate a new project with stars.

**Request**:
```json
{
  "name": "MyGalaxy",
  "num_stars": 100,
  "distribution_type": "sphere",
  "radius_ly": 100.0
}
```

**Response**:
```json
{
  "project_id": "uuid-here",
  "name": "MyGalaxy",
  "num_star_systems": 100,
  "num_stars_total": 126,
  "solo_systems": 74,
  "binary_systems": 25,
  "trinary_systems": 1,
  "generation_time_ms": 523
}
```

#### GET /api/projects
List all projects.

**Response**:
```json
{
  "projects": [
    {
      "id": "uuid-1",
      "name": "MyGalaxy",
      "num_stars": 126,
      "created_at": "2025-11-02T10:30:00Z"
    }
  ]
}
```

#### GET /api/projects/{id}/stars
Get stars for a project.

**Response**:
```json
{
  "project_id": "uuid-1",
  "project_name": "MyGalaxy",
  "stars": [
    {
      "system_name": "S0001",
      "stars": [
        {
          "name": "S0001a",
          "spectral_class": "G",
          "temperature_k": 5778,
          "position": {"x": 10.5, "y": 20.3, "z": -5.7}
        }
      ]
    }
  ]
}
```

## Blazor WASM Structure

```
StellarForge.Web/
├── StellarForge.Web.csproj
├── wwwroot/
│   ├── index.html
│   ├── css/
│   │   └── app.css
│   └── js/
│       └── interop.js
├── Pages/
│   ├── Index.razor              # Home page
│   ├── GenerateStars.razor      # Star generation form
│   └── ViewProject.razor        # View generated stars
├── Components/
│   ├── StarGenerationForm.razor # Form component
│   └── StarsList.razor          # List component
├── Services/
│   ├── ApiService.cs            # HTTP client for backend API
│   └── StateService.cs          # App state management
├── Models/
│   ├── GenerateRequest.cs
│   ├── Project.cs
│   └── Star.cs
├── Program.cs
└── _Imports.razor
```

### UI Layout

#### Star Generation Form
```
┌──────────────────────────────────────────────┐
│  Generate Stars                              │
├──────────────────────────────────────────────┤
│                                              │
│  Project Name: [___________________]         │
│                                              │
│  Number of Stars: [_______] (1-10,000)       │
│                                              │
│  Distribution Type:                          │
│    ○ Cube   ● Sphere                         │
│                                              │
│  [ Sphere selected ]                         │
│    Radius (LY): [_______]                    │
│                                              │
│  [ Cube not selected ]                       │
│    Size X (LY): [_______]                    │
│    Size Y (LY): [_______]                    │
│    Size Z (LY): [_______]                    │
│                                              │
│  [  Generate Stars  ]                        │
│                                              │
└──────────────────────────────────────────────┘
```

#### Results Display
```
┌──────────────────────────────────────────────┐
│  Project: MyGalaxy                           │
├──────────────────────────────────────────────┤
│  Statistics:                                 │
│    Total Systems: 100                        │
│    Total Stars: 126                          │
│    Solo Systems: 74 (74%)                    │
│    Binary Systems: 25 (25%)                  │
│    Trinary Systems: 1 (1%)                   │
│                                              │
│  Star Systems:                               │
│  ┌────────────────────────────────────────┐ │
│  │ S0001 (Binary)                         │ │
│  │   S0001a - G5V (5,650K) @ (10, 20, 5) │ │
│  │   S0001b - K2V (4,800K) @ (10, 20, 5) │ │
│  ├────────────────────────────────────────┤ │
│  │ S0002 (Solo)                           │ │
│  │   S0002 - M3V (3,200K) @ (15, 8, -3)  │ │
│  └────────────────────────────────────────┘ │
└──────────────────────────────────────────────┘
```

## Nginx Configuration

```nginx
# nginx.conf (Windows: C:\nginx\conf\nginx.conf)

worker_processes 1;

events {
    worker_connections 1024;
}

http {
    include mime.types;
    default_type application/octet-stream;

    upstream backend {
        server 127.0.0.1:8080;
    }

    server {
        listen 80;
        server_name localhost;

        # Serve Blazor WASM static files
        location / {
            root C:/projects/stellarforge/blazor/StellarForge.Web/bin/Release/net8.0/publish/wwwroot;
            try_files $uri $uri/ /index.html;
            add_header Cache-Control "no-cache";
        }

        # Proxy API requests to Rust backend
        location /api/ {
            proxy_pass http://backend;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection 'upgrade';
            proxy_set_header Host $host;
            proxy_cache_bypass $http_upgrade;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
}
```

## Star Generation Algorithm

### Pseudocode

```rust
fn generate_stars(params: GenerateParams) -> Result<Vec<StarSystem>> {
    let mut systems = Vec::new();

    for i in 0..params.num_stars {
        // Generate position
        let position = match params.distribution_type {
            DistributionType::Cube => generate_cube_position(params),
            DistributionType::Sphere => generate_sphere_position(params),
        };

        // Determine system type (solo, binary, trinary)
        let system_type = weighted_random_system_type();

        // Generate system name
        let system_name = format!("S{:04}", i + 1);

        // Create stars for this system
        let stars = match system_type {
            SystemType::Solo => vec![generate_star(&system_name, None)],
            SystemType::Binary => vec![
                generate_star(&system_name, Some('a')),
                generate_star(&system_name, Some('b')),
            ],
            SystemType::Trinary => vec![
                generate_star(&system_name, Some('a')),
                generate_star(&system_name, Some('b')),
                generate_star(&system_name, Some('c')),
            ],
        };

        systems.push(StarSystem {
            name: system_name,
            system_type,
            position,
            stars,
        });
    }

    Ok(systems)
}

fn generate_sphere_position(radius: f64) -> Position3D {
    // Random distance from center
    let r = rand::random::<f64>().powf(1.0/3.0) * radius; // Uniform distribution

    // Random azimuth (0 to 2π)
    let theta = rand::random::<f64>() * 2.0 * PI;

    // Random polar angle (0 to π)
    let phi = (rand::random::<f64>() * 2.0 - 1.0).acos();

    // Convert spherical to Cartesian
    Position3D {
        x: r * phi.sin() * theta.cos(),
        y: r * phi.sin() * theta.sin(),
        z: r * phi.cos(),
    }
}

fn generate_star(system_name: &str, suffix: Option<char>) -> Star {
    let name = match suffix {
        Some(c) => format!("{}{}", system_name, c),
        None => system_name.to_string(),
    };

    // Weighted random spectral class
    let spectral_class = weighted_random_spectral_class();

    // Generate properties based on spectral class
    let (temp, mass, radius, luminosity) = spectral_class_properties(spectral_class);

    Star {
        name,
        spectral_class,
        temperature_k: temp,
        mass_solar: mass,
        radius_solar: radius,
        luminosity_solar: luminosity,
    }
}

fn weighted_random_spectral_class() -> char {
    // Weights: M=20, K=15, G=11, F=7, A=4, B=2, O=1
    let weights = [1, 2, 4, 7, 11, 15, 20];
    let classes = ['O', 'B', 'A', 'F', 'G', 'K', 'M'];

    weighted_choice(&classes, &weights)
}
```

## Implementation Steps

### 1. Database Setup
- [ ] Install PostgreSQL 18 with PostGIS
- [ ] Run schema creation scripts
- [ ] Test connection from Rust

### 2. Rust Backend
- [ ] Create Actix-Web project
- [ ] Implement database connection pool
- [ ] Create data models (Project, StarSystem, Star)
- [ ] Implement star generation algorithm
- [ ] Create API endpoints
- [ ] Add CORS middleware
- [ ] Test with curl/Postman

### 3. Blazor Frontend
- [ ] Create Blazor WASM project
- [ ] Design UI components
- [ ] Implement form validation
- [ ] Create API service client
- [ ] Wire up form to backend
- [ ] Display results

### 4. Nginx Setup
- [ ] Install Nginx on Windows
- [ ] Configure static file serving
- [ ] Configure reverse proxy
- [ ] Test end-to-end workflow

### 5. Testing
- [ ] Test cube distribution
- [ ] Test sphere distribution
- [ ] Verify star system type ratios (74/25/1)
- [ ] Verify spectral class distribution
- [ ] Test with various star counts (1, 10, 100, 1000, 10000)

## Success Criteria

- [ ] User can access Blazor UI at http://localhost
- [ ] User can input generation parameters
- [ ] Stars generate with correct distribution
- [ ] Stars saved to PostgreSQL successfully
- [ ] Results display correctly in UI
- [ ] System type ratios match specification (74/25/1)
- [ ] Spectral class distribution follows 20x cool-to-hot ratio
- [ ] Star names follow S#### format with a/b/c suffixes

## Next Phase Preview

Phase 2 will add:
- 3D visualization of generated stars
- Ability to view/edit individual stars
- Export to various formats
- More complex generation parameters (density zones, spectral class constraints)

---

*StellarForge Phase 1 - Laying the foundation for procedural stellar cartography*
