# StellarForge Phase 1 - Deployment Guide

## Prerequisites

### Required Software
- **PostgreSQL 18+** with PostGIS extension
- **Rust 1.70+** (already installed)
- **.NET 9 SDK** (already installed)
- **Nginx** (for production deployment)

### Database Setup

You can use the automated script or run commands manually.

#### Automated (Recommended)

```powershell
cd D:\\projects\\stellarforge
./setup_database.ps1
```

#### Manual

1. **Start PostgreSQL** (if not running)
2. **Open PowerShell as Administrator**
3. **Run the following commands**:

```powershell
# Set password environment variable
$env:PGPASSWORD = "Beta5357"

# Create database
& "C:\Program Files\PostgreSQL\18\bin\psql.exe" -U postgres -c "CREATE DATABASE stellarforge;"

# Create extensions and schema
& "C:\Program Files\PostgreSQL\18\bin\psql.exe" -U postgres -d stellarforge -f D:\projects\stellarforge\sql\01_create_database.sql

# Create tables
& "C:\Program Files\PostgreSQL\18\bin\psql.exe" -U postgres -d stellarforge -f D:\projects\stellarforge\sql\02_create_tables.sql

# Create helper functions
& "C:\Program Files\PostgreSQL\18\bin\psql.exe" -U postgres -d stellarforge -f D:\projects\stellarforge\sql\03_helper_functions.sql

# Verify setup
& "C:\Program Files\PostgreSQL\18\bin\psql.exe" -U postgres -d stellarforge -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'stellar';"
```

You should see **3 tables** in the stellar schema.

#### Alternative: Using PgAdmin

1. Open PgAdmin
2. Create a new database called `stellarforge`
3. Open Query Tool for the database
4. Execute the contents of each SQL file in order:
   - `sql/01_create_database.sql`
   - `sql/02_create_tables.sql`
   - `sql/03_helper_functions.sql`

## Development Setup

### 1. Start the Rust Backend

```bash
cd D:\projects\stellarforge\backend
cargo run
```

The backend will start on **http://localhost:8080**

Verify it's running:
```bash
curl http://localhost:8080/api/health
```

### 2. Start the Blazor Frontend (Development Mode)

```bash
cd D:\projects\stellarforge\blazor\StellarForge.Web
dotnet watch run
```

The frontend will start on **http://localhost:5000** (or similar)

Open your browser and navigate to the displayed URL.

## Production Deployment with Nginx

### 1. Build Blazor for Production

```bash
cd D:\projects\stellarforge\blazor\StellarForge.Web
dotnet publish -c Release -o D:\projects\stellarforge\publish
```

### 2. Install and Configure Nginx

#### Download Nginx for Windows
1. Download from https://nginx.org/en/download.html (Stable version)
2. Extract to `C:\nginx`

#### Create Nginx Configuration

Create/edit `C:\nginx\conf\nginx.conf`:

```nginx
worker_processes 1;

events {
    worker_connections 1024;
}

http {
    include mime.types;
    default_type application/octet-stream;

    # Enable gzip compression
    gzip on;
    gzip_types text/css application/javascript application/json;

    upstream backend {
        server 127.0.0.1:8080;
    }

    server {
        listen 80;
        listen 443 ssl;
        server_name localhost 127.0.0.1;

        # Self-signed SSL certificate (for development)
        ssl_certificate C:/nginx/conf/cert.pem;
        ssl_certificate_key C:/nginx/conf/cert.key;

        # Serve Blazor WASM app at /stellarforge
        location /stellarforge {
            alias D:/projects/stellarforge/publish/wwwroot;
            try_files $uri $uri/ /stellarforge/index.html;

            # MIME types
            types {
                application/wasm wasm;
            }

            # Cache static assets
            location ~* \.(wasm|dll|pdb|dat|blat)$ {
                expires 1y;
                add_header Cache-Control "public, immutable";
            }
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

#### Update Blazor Base Path

Edit `D:\projects\stellarforge\blazor\StellarForge.Web\wwwroot\index.html`:

Change:
```html
<base href="/" />
```

To:
```html
<base href="/stellarforge/" />
```

#### Generate Self-Signed SSL Certificate (Development Only)

```bash
cd C:\nginx\conf
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout cert.key -out cert.pem -subj "/C=US/ST=State/L=City/O=Org/CN=localhost"
```

Or use PowerShell:
```powershell
$cert = New-SelfSignedCertificate -DnsName "localhost", "127.0.0.1" -CertStoreLocation "cert:\LocalMachine\My"
```

### 3. Start Services

#### Start Backend (in one terminal)
```bash
cd D:\projects\stellarforge\backend
cargo run --release
```

#### Start Nginx (in another terminal)
```bash
cd C:\nginx
.\nginx.exe
```

Or install as Windows service (optional).

### 4. Access the Application

Open your browser and navigate to:

**HTTPS**: https://127.0.0.1/stellarforge
**HTTP**: http://127.0.0.1/stellarforge

(You'll get a security warning for the self-signed certificate - this is normal for development)

## Testing

### 1. Generate Test Stars

1. Open https://127.0.0.1/stellarforge
2. Enter project name: "TestGalaxy"
3. Set number of stars: 100
4. Select distribution: Sphere
5. Set radius: 100 LY
6. Click "Generate Stars"

### 2. Verify Results

You should see:
- Generation complete message
- ~74 solo systems
- ~25 binary systems
- ~1 trinary system
- ~126 total stars
- Generation time (< 1 second typically)

### 3. Check Database

```sql
-- Connect to stellarforge database
SELECT COUNT(*) FROM stellar.projects;
SELECT COUNT(*) FROM stellar.star_systems;
SELECT COUNT(*) FROM stellar.stars;

-- View a sample project
SELECT * FROM stellar.projects LIMIT 1;

-- View sample stars
SELECT name, spectral_class, temperature_k
FROM stellar.stars
ORDER BY name
LIMIT 10;
```

## Screenshots

Screenshots for testing and documentation should be saved to:
```
D:\dropbox\screenshots\
```

Recommended screenshots:
1. `stellarforge-home-page.png` - Home page with form
2. `stellarforge-generation-result.png` - After generating stars
3. `stellarforge-projects-list.png` - Recent projects table
4. `stellarforge-database-query.png` - Database query results

## Troubleshooting

### Backend won't start
- Check PostgreSQL is running
- Verify database exists: `psql -U postgres -l | grep stellarforge`
- Check connection string in `backend/.env`

### Frontend shows "Backend Offline"
- Verify backend is running: `curl http://localhost:8080/api/health`
- Check CORS configuration in backend

### Nginx won't start
- Check port 80/443 aren't in use
- Run `nginx -t` to test configuration
- Check nginx error logs: `C:\nginx\logs\error.log`

### WASM files won't load
- Check MIME types in nginx.conf
- Verify base href in index.html matches nginx location
- Clear browser cache

## Production Notes

For actual production deployment:
1. Use a proper SSL certificate (Let's Encrypt, commercial CA)
2. Configure database backups
3. Set up monitoring and logging
4. Use environment-specific configuration
5. Consider using a reverse proxy manager like Caddy or Traefik
6. Set up proper authentication/authorization

## Next Steps

Phase 2 will add:
- 3D visualization of generated stars
- Ability to view/edit individual stars
- Export functionality (CSV, JSON)
- More generation parameters
- Planet generation

---

**StellarForge Phase 1** - Production-ready star generation system
