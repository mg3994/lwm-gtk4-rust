# Windows Setup Guide for GTK4 Development

## Quick Start (Recommended Method)

### Step 1: Install MSYS2

1. Download MSYS2 installer from: https://www.msys2.org/
2. Run the installer (e.g., `msys2-x86_64-20240113.exe`)
3. Follow the installation wizard (default location: `C:\msys64`)
4. After installation, MSYS2 terminal will open automatically

### Step 2: Update MSYS2

In the MSYS2 terminal, run:
```bash
pacman -Syu
```

If it asks to close the terminal, close it and reopen **MSYS2 MINGW64** (not MSYS2 MSYS).

Run the update again:
```bash
pacman -Su
```

### Step 3: Install GTK4 and Development Tools

In the **MSYS2 MINGW64** terminal:
```bash
pacman -S mingw-w64-x86_64-gtk4
pacman -S mingw-w64-x86_64-pkg-config
pacman -S mingw-w64-x86_64-gcc
pacman -S mingw-w64-x86_64-toolchain
```

Type `Y` and press Enter when prompted.

### Step 4: Add MSYS2 to Windows PATH

1. Open **System Properties** → **Environment Variables**
2. Under **System Variables**, find and edit **Path**
3. Click **New** and add: `C:\msys64\mingw64\bin`
4. Click **OK** to save
5. **Restart your terminal/PowerShell/IDE**

### Step 5: Verify Installation

Open a **new PowerShell** window and run:
```powershell
pkg-config --modversion gtk4
```

You should see a version number like `4.12.0` or similar.

Also verify:
```powershell
gcc --version
pkg-config --version
```

### Step 6: Install Rust (if not already installed)

Download and run from: https://rustup.rs/

Or in PowerShell:
```powershell
# Download rustup-init.exe and run it
# Follow the prompts (default installation is fine)
```

After installation, restart your terminal and verify:
```powershell
rustc --version
cargo --version
```

### Step 7: Build the Project

Navigate to your project:
```powershell
cd C:\Users\manis\Desktop\gtk4
```

Build:
```powershell
cargo build --release
```

**First build will take 5-10 minutes** as it downloads and compiles all dependencies.

Run:
```powershell
cargo run --release
```

## Alternative: Using MSYS2 Terminal for Everything

If you prefer to work entirely in MSYS2:

1. Open **MSYS2 MINGW64** terminal
2. Install Rust in MSYS2:
   ```bash
   pacman -S mingw-w64-x86_64-rust
   ```
3. Navigate to project:
   ```bash
   cd /c/Users/manis/Desktop/gtk4
   ```
4. Build and run:
   ```bash
   cargo build --release
   cargo run --release
   ```

## Troubleshooting

### Issue: "pkg-config: command not found"

**Solution:**
- Make sure you added `C:\msys64\mingw64\bin` to PATH
- Restart your terminal/IDE after adding to PATH
- Verify PATH was added: `echo $env:PATH` (should include msys64)

### Issue: "Package gtk4 was not found"

**Solution:**
- Reinstall GTK4: `pacman -S mingw-w64-x86_64-gtk4`
- Check if pkg-config can find it: `pkg-config --list-all | Select-String gtk`

### Issue: Build fails with "linker error"

**Solution:**
- Install the full toolchain: `pacman -S mingw-w64-x86_64-toolchain`
- Make sure you're using MINGW64 version, not MSYS2

### Issue: "The process cannot access the file" (os error 32)

**Solution:**
- Close any running instances of your app
- Close VS Code or other IDEs that might lock files
- Run: `cargo clean`
- Try building again

### Issue: Application window doesn't show

**Solution:**
- Check if `resources/style.css` exists
- Run from the project root directory
- Check console for error messages

### Issue: Slow compilation

**Solution:**
- First build is always slow (5-10 minutes)
- Subsequent builds are much faster (30 seconds)
- Use `cargo build` (debug) for faster iteration during development
- Use `cargo build --release` only for final builds

## Development Workflow

### Fast Iteration

For quick development cycles:
```powershell
# Use debug build (faster compilation)
cargo run

# Watch for changes and auto-rebuild (install cargo-watch first)
cargo install cargo-watch
cargo watch -x run
```

### Checking for Errors

```powershell
# Check code without building
cargo check

# Check with clippy (linter)
cargo clippy
```

### Formatting

```powershell
# Format code
cargo fmt
```

## IDE Setup

### VS Code

Install extensions:
- **rust-analyzer** - Rust language support
- **CodeLLDB** - Debugging support

Add to `.vscode/settings.json`:
```json
{
    "rust-analyzer.cargo.target": "x86_64-pc-windows-gnu",
    "terminal.integrated.env.windows": {
        "PATH": "C:\\msys64\\mingw64\\bin;${env:PATH}"
    }
}
```

### CLion / RustRover

1. Go to **Settings** → **Build, Execution, Deployment** → **Toolchains**
2. Add MinGW toolchain pointing to `C:\msys64\mingw64`
3. Set as default toolchain

## Next Steps

Once everything is working:

1. **Explore the code** - Check out `src/main.rs` to see how views are created
2. **Customize styling** - Edit `resources/style.css` to change colors/fonts
3. **Add features** - Extend the views with new functionality
4. **Connect backend** - Integrate with a REST API or database

## Useful Resources

- **GTK4 Documentation**: https://docs.gtk.org/gtk4/
- **gtk-rs Book**: https://gtk-rs.org/gtk4-rs/stable/latest/book/
- **Rust GTK4 Examples**: https://github.com/gtk-rs/gtk4-rs/tree/master/examples
- **MSYS2 Packages**: https://packages.msys2.org/

## Getting Help

If you encounter issues:

1. Check the error message carefully
2. Search for the error on GitHub issues: https://github.com/gtk-rs/gtk4-rs/issues
3. Ask on Rust Discord: https://discord.gg/rust-lang
4. Check GTK forums: https://discourse.gnome.org/

---

**Happy coding! 🚀**
