param(
    [switch]$SkipSetup
)

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
Set-Location $RepoRoot

# Read version from Cargo.toml
$cargoToml = Get-Content (Join-Path $RepoRoot "Cargo.toml") -Raw
if ($cargoToml -match '(?m)^version\s*=\s*"([^"]+)"') {
    $Version = $Matches[1]
} else {
    throw "Could not parse workspace.package.version from Cargo.toml"
}

Write-Host "=== Building Blanc Engine v$Version ===" -ForegroundColor Cyan

# Resolve exact Cargo target directory (respects .cargo/config.toml)
$metaJson = cargo metadata --no-deps --format-version 1 | ConvertFrom-Json
$TargetDir = $metaJson.target_directory
$ReleaseDir = Join-Path $TargetDir "release"

# 1. Build runtime & installer first
Write-Host "`n[1/4] Building blanc_runtime and blanc_installer (release)..." -ForegroundColor Yellow
cargo build --release -p blanc_runtime -p blanc_installer
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$RuntimeExe = Join-Path $ReleaseDir "blanc_runtime.exe"
$InstallerExe = Join-Path $ReleaseDir "blanc_installer.exe"
if (-not (Test-Path $RuntimeExe)) { throw "Missing $RuntimeExe" }
if (-not (Test-Path $InstallerExe)) { throw "Missing $InstallerExe" }

# 2. Build Portable single-file BlancEngine (embeds runtime + installer)
Write-Host "`n[2/4] Building Portable single-file BlancEngine-$Version-Portable.exe..." -ForegroundColor Yellow
$env:BLANC_BUNDLE_RUNTIME_PATH = $RuntimeExe
$env:BLANC_BUNDLE_INSTALLER_PATH = $InstallerExe
cargo build --release -p blanc_editor --features bundle-binaries
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$DistDir = Join-Path $RepoRoot "dist"
New-Item -ItemType Directory -Force -Path $DistDir | Out-Null
$PortableOut = Join-Path $DistDir "BlancEngine-$Version-Portable.exe"
Copy-Item (Join-Path $ReleaseDir "blanc_editor.exe") $PortableOut -Force
Write-Host "Created Portable: $PortableOut" -ForegroundColor Green

# 3. Build unbundled blanc_editor for Inno Setup staging (keeps installed dir lean)
if (-not $SkipSetup) {
    Write-Host "`n[3/4] Staging files for Windows Setup Installer..." -ForegroundColor Yellow
    Remove-Item Env:BLANC_BUNDLE_RUNTIME_PATH -ErrorAction SilentlyContinue
    Remove-Item Env:BLANC_BUNDLE_INSTALLER_PATH -ErrorAction SilentlyContinue
    cargo build --release -p blanc_editor
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    $StagingDir = Join-Path $DistDir "staging"
    New-Item -ItemType Directory -Force -Path $StagingDir | Out-Null
    Copy-Item (Join-Path $ReleaseDir "blanc_editor.exe") (Join-Path $StagingDir "BlancEngine.exe") -Force
    Copy-Item $RuntimeExe (Join-Path $StagingDir "blanc_runtime.exe") -Force
    Copy-Item $InstallerExe (Join-Path $StagingDir "blanc_installer.exe") -Force

    # 4. Run Inno Setup 6 if installed
    $IsccPath = @(
        "C:\Program Files (x86)\Inno Setup 6\ISCC.exe",
        "C:\Program Files\Inno Setup 6\ISCC.exe",
        "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe"
    ) | Where-Object { Test-Path $_ } | Select-Object -First 1

    if ($IsccPath) {
        Write-Host "`n[4/4] Building BlancEngine-$Version-Setup.exe with Inno Setup..." -ForegroundColor Yellow
        $IssPath = Join-Path $RepoRoot "installer\blanc_engine.iss"
        & $IsccPath "/DMyAppVersion=$Version" "/DStagingDir=$StagingDir" "/DOutputDir=$DistDir" $IssPath
        if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
        $SetupOut = Join-Path $DistDir "BlancEngine-$Version-Setup.exe"
        Write-Host "Created Setup: $SetupOut" -ForegroundColor Green
    } else {
        Write-Host "`n[4/4] Inno Setup 6 (ISCC.exe) not found." -ForegroundColor DarkYellow
        Write-Host "Install Inno Setup 6 via: winget install JRSoftware.InnoSetup" -ForegroundColor DarkYellow
        Write-Host "Then re-run this script to generate dist\BlancEngine-$Version-Setup.exe." -ForegroundColor DarkYellow
    }
}

Write-Host "`nDone! Outputs in: $DistDir" -ForegroundColor Cyan
