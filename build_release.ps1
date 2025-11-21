# --------------------------------------------------------------
# build_release.ps1 – Build LinkWithMentor (release) with MSYS2 pkg-config
# --------------------------------------------------------------

# 1️⃣ Add MSYS2 mingw64 binaries to the PATH (if they exist)
$msysBin = "C:\msys64\mingw64\bin"
if (Test-Path $msysBin) {
    $env:PATH = "$msysBin;$env:PATH"
    Write-Host "✅ Added $msysBin to PATH" -ForegroundColor Green
} else {
    Write-Host "⚠️  $msysBin not found – make sure MSYS2 is installed." -ForegroundColor Yellow
    exit 1
}

# 2️⃣ Tell pkg-config where the .pc files live
$pkgConfigPath = "C:\msys64\mingw64\lib\pkgconfig"
if (Test-Path $pkgConfigPath) {
    $env:PKG_CONFIG_PATH = $pkgConfigPath
    Write-Host "✅ PKG_CONFIG_PATH set to $pkgConfigPath" -ForegroundColor Green
} else {
    Write-Host "⚠️  $pkgConfigPath not found – pkg-config may not have the needed files." -ForegroundColor Yellow
    exit 1
}

# 3️⃣ Allow system CFLAGS (required by glib-sys)
$env:PKG_CONFIG_ALLOW_SYSTEM_CFLAGS = "1"

# 4️⃣ Run the release build
Write-Host "\n🚀 Building LinkWithMentor (release)..." -ForegroundColor Cyan
cargo build --release

# 5️⃣ Report the result
if ($LASTEXITCODE -eq 0) {
    Write-Host "\n✅ Build succeeded! Executable is at:" -ForegroundColor Green
    Write-Host "   target\release\linkwithmentor.exe" -ForegroundColor White
} else {
    Write-Host "\n❌ Build failed. See the error messages above." -ForegroundColor Red
}
