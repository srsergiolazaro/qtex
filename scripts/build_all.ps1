# Build script for qtex (Windows native + Cross-compilation)
# Requirements: 
# 1. Rust installed
# 2. 'cross' installed (cargo install cross)
# 3. Docker running (for Linux/macOS cross-compilation)

$Version = (Get-Content Cargo.toml | Select-String "^version = ""(.*)""" | ForEach-Object { $_.Matches.Groups[1].Value })
Write-Host "🚀 Building qtex v$Version for multiple platforms..." -ForegroundColor Cyan
Write-Host "ℹ️ Using rustls-tls to avoid OpenSSL issues on Linux targets." -ForegroundColor Gray

$ReleaseDir = "releases"
if (!(Test-Path $ReleaseDir)) { New-Item -ItemType Directory -Path $ReleaseDir }

# Function to check build status
function Check-Build {
    param($Path, $Target)
    if (Test-Path $Path) {
        Write-Host "✅ Success: $Target" -ForegroundColor Green
    } else {
        Write-Host "❌ Failed: $Target (Binary not found at $Path)" -ForegroundColor Red
        exit 1
    }
}

# 1. Linux (via cross/Docker) - VERY COST EFFECTIVE TO DO LOCALLY
if (Get-Command cross -ErrorAction SilentlyContinue) {
    Write-Host "`n📦 Building Linux x64 (glibc) via cross..." -ForegroundColor Green
    cross build --release --target x86_64-unknown-linux-gnu
    if (Test-Path "target\x86_64-unknown-linux-gnu\release\qtex") {
        Copy-Item "target\x86_64-unknown-linux-gnu\release\qtex" "$ReleaseDir\qtex-linux-x64"
        Write-Host "✅ Success: Linux x64" -ForegroundColor Green
    }

    Write-Host "`n📦 Building Linux x64 (musl) via cross..." -ForegroundColor Green
    cross build --release --target x86_64-unknown-linux-musl
    if (Test-Path "target\x86_64-unknown-linux-musl\release\qtex") {
        Copy-Item "target\x86_64-unknown-linux-musl\release\qtex" "$ReleaseDir\qtex-linux-x64-musl"
        Write-Host "✅ Success: Linux x64 musl" -ForegroundColor Green
    }
} else {
    Write-Host "`n⚠️  'cross' not found. Skipping Linux builds. (cargo install cross)" -ForegroundColor Yellow
}

# 2. Windows (Native)
Write-Host "`n📦 Building Windows x64..." -ForegroundColor Green
cargo build --release --target x86_64-pc-windows-msvc
Copy-Item "target\x86_64-pc-windows-msvc\release\qtex.exe" "$ReleaseDir\qtex-windows-x64.exe"
Check-Build "$ReleaseDir\qtex-windows-x64.exe" "Windows x64"

Write-Host "`n✅ Local builds (Windows/Linux) completed in $ReleaseDir/" -ForegroundColor Green

# 3. Optional: Upload to GitHub Release
Write-Host "`n🚀 Ready to publish?" -ForegroundColor Cyan
Write-Host "1. git add Cargo.toml; git commit -m 'release: v$Version'; git tag v$Version; git push origin v$Version"
Write-Host "2. Wait for GitHub Actions to build macOS."
Write-Host "3. Run the following command to upload these local builds to the release:" -ForegroundColor Gray

if (Get-Command gh -ErrorAction SilentlyContinue) {
    Write-Host "   gh release upload v$Version (Get-ChildItem $ReleaseDir/* | ForEach-Object {`"$($_.FullName)`"}) --clobber" -ForegroundColor Yellow
} else {
    Write-Host "   (Install GitHub CLI 'gh' to automate this step)" -ForegroundColor DarkGray
}
