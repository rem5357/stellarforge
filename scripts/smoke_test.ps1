$ErrorActionPreference = "Stop"

Write-Host "Starting StellarForge backend for smoke test..." -ForegroundColor Cyan
$env:DATABASE_URL = $env:DATABASE_URL -as [string]
if (-not $env:DATABASE_URL) {
  $env:DATABASE_URL = "postgres://postgres:Beta5357@localhost:5432/stellarforge"
}

$backendDir = Join-Path $PSScriptRoot "..\backend"

$proc = Start-Process -FilePath "cargo" -ArgumentList "run --release" -WorkingDirectory $backendDir -PassThru -WindowStyle Hidden
Start-Sleep -Seconds 3

try {
  $resp = Invoke-WebRequest -Uri 'http://127.0.0.1:8080/api/health' -UseBasicParsing -TimeoutSec 5
  Write-Host ("Health response status: {0}" -f $resp.StatusCode) -ForegroundColor Green
  Write-Host $resp.Content
} catch {
  Write-Host ("Health check failed: {0}" -f $_.Exception.Message) -ForegroundColor Red
}

try {
  Get-Process | Where-Object { $_.ProcessName -like 'stellarforge-backend*' } | ForEach-Object { Stop-Process -Id $_.Id -Force; Write-Host ("Stopped process {0} ({1})" -f $_.ProcessName, $_.Id) -ForegroundColor Yellow }
} catch {}

Write-Host "Smoke test complete." -ForegroundColor Cyan
