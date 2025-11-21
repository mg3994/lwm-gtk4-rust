# Quick Build Script for LinkWithMentor

Write-Host "🎓 LinkWithMentor - GTK4 Social Media App" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

# Check if GTK4 is installed
Write-Host "Checking GTK4 installation..." -ForegroundColor Yellow
try {
    $gtk_version = pkg-config --modversion gtk4 2>$null
    if ($gtk_version) {
        Write-Host "✓ GTK4 $gtk_version found!" -ForegroundColor Green
    } else {
        throw "GTK4 not found"
    }
} catch {
    Write-Host "✗ GTK4 not found!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install GTK4 first. See WINDOWS_SETUP.md for instructions." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Quick install with MSYS2:" -ForegroundColor Cyan
    Write-Host "  1. Download MSYS2 from https://www.msys2.org/" -ForegroundColor White
    Write-Host "  2. Open MSYS2 MINGW64 terminal" -ForegroundColor White
    Write-Host "  3. Run: pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-pkg-config" -ForegroundColor White
    Write-Host "  4. Add C:\msys64\mingw64\bin to your PATH" -ForegroundColor White
    Write-Host "  5. Restart this terminal and run this script again" -ForegroundColor White
    Write-Host ""
    exit 1
}

# Check if Rust is installed
Write-Host "Checking Rust installation..." -ForegroundColor Yellow
try {
    $rust_version = rustc --version 2>$null
    if ($rust_version) {
        Write-Host "✓ $rust_version" -ForegroundColor Green
    } else {
        throw "Rust not found"
    }
} catch {
    Write-Host "✗ Rust not found!" -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install Rust from https://rustup.rs/" -ForegroundColor Yellow
    Write-Host ""
    exit 1
}

Write-Host ""
Write-Host "All dependencies found! 🎉" -ForegroundColor Green
Write-Host ""

# Ask user what to do
Write-Host "What would you like to do?" -ForegroundColor Cyan
Write-Host "  [1] Build (debug mode - faster compilation)" -ForegroundColor White
Write-Host "  [2] Build and Run (debug mode)" -ForegroundColor White
Write-Host "  [3] Build (release mode - optimized)" -ForegroundColor White
Write-Host "  [4] Build and Run (release mode)" -ForegroundColor White
Write-Host "  [5] Clean build artifacts" -ForegroundColor White
Write-Host "  [6] Check code (no build)" -ForegroundColor White
Write-Host ""

$choice = Read-Host "Enter your choice (1-6)"

switch ($choice) {
    "1" {
        Write-Host ""
        Write-Host "Building in debug mode..." -ForegroundColor Yellow
        cargo build
    }
    "2" {
        Write-Host ""
        Write-Host "Building and running in debug mode..." -ForegroundColor Yellow
        cargo run
    }
    "3" {
        Write-Host ""
        Write-Host "Building in release mode (this will take longer)..." -ForegroundColor Yellow
        cargo build --release
    }
    "4" {
        Write-Host ""
        Write-Host "Building and running in release mode..." -ForegroundColor Yellow
        cargo run --release
    }
    "5" {
        Write-Host ""
        Write-Host "Cleaning build artifacts..." -ForegroundColor Yellow
        cargo clean
        Write-Host "✓ Clean complete!" -ForegroundColor Green
    }
    "6" {
        Write-Host ""
        Write-Host "Checking code..." -ForegroundColor Yellow
        cargo check
    }
    default {
        Write-Host ""
        Write-Host "Invalid choice. Exiting." -ForegroundColor Red
        exit 1
    }
}

Write-Host ""
if ($LASTEXITCODE -eq 0) {
    Write-Host "✓ Success!" -ForegroundColor Green
} else {
    Write-Host "✗ Build failed. Check the errors above." -ForegroundColor Red
    Write-Host ""
    Write-Host "Common issues:" -ForegroundColor Yellow
    Write-Host "  - File lock errors: Close any running instances and try 'cargo clean'" -ForegroundColor White
    Write-Host "  - Missing dependencies: Make sure GTK4 is properly installed" -ForegroundColor White
    Write-Host "  - Path issues: Verify C:\msys64\mingw64\bin is in your PATH" -ForegroundColor White
    Write-Host ""
    Write-Host "See WINDOWS_SETUP.md for detailed troubleshooting." -ForegroundColor Cyan
}

Write-Host ""
