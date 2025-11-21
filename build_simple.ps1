$env:PATH = "C:\msys64\mingw64\bin;" + $env:PATH
Write-Host "Building LinkWithMentor..."
cargo build
