# GitHub Workflows Guide

This document explains the CI/CD workflows set up for LinkWithMentor.

## 🔄 Workflows Overview

### 1. **CI Workflow** (`.github/workflows/ci.yml`)

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main`

**Jobs:**

#### Check
- Runs `cargo check` to verify code compiles
- Uses MSYS2 for GTK4 dependencies
- Caches dependencies for faster builds

#### Test
- Runs `cargo test` to execute all tests
- Ensures code functionality

#### Clippy
- Runs `cargo clippy` for linting
- Fails on warnings (`-D warnings`)
- Ensures code quality

#### Format
- Runs `cargo fmt --check`
- Ensures consistent code formatting
- Runs on Ubuntu (faster, no GTK4 needed for formatting)

**Status:** ✅ Runs on every push and PR

---

### 2. **Release Workflow** (`.github/workflows/release.yml`)

**Triggers:**
- Push tags matching `v*` (e.g., `v1.0.0`)
- Manual workflow dispatch

**Process:**

1. **Setup Environment**
   - Installs MSYS2 with GTK4
   - Sets up Rust toolchain
   - Configures caching

2. **Build Release**
   - Compiles with `--release` flag
   - Targets `x86_64-pc-windows-gnu`
   - Optimized for performance

3. **Bundle Dependencies**
   - Copies executable
   - Includes `resources/` folder
   - Bundles GTK4 DLLs
   - Adds documentation

4. **Create Archive**
   - Creates ZIP file: `LinkWithMentor-Windows-x64.zip`
   - Ready for distribution

5. **GitHub Release**
   - Creates release on GitHub
   - Uploads ZIP artifact
   - Generates release notes automatically

**Status:** 🚀 Creates releases on tags

---

## 📦 Creating a Release

### Step 1: Prepare Release

1. **Update version** in `Cargo.toml`:
   ```toml
   [package]
   version = "1.0.0"
   ```

2. **Update CHANGELOG** (if you have one)

3. **Commit changes:**
   ```bash
   git add Cargo.toml
   git commit -m "chore: bump version to 1.0.0"
   git push origin main
   ```

### Step 2: Create and Push Tag

```bash
# Create annotated tag
git tag -a v1.0.0 -m "Release v1.0.0: Initial release with 11 screens"

# Push tag to GitHub
git push origin v1.0.0
```

### Step 3: Wait for Workflow

1. Go to **Actions** tab on GitHub
2. Watch the **Build and Release** workflow
3. Wait for completion (~10-15 minutes)

### Step 4: Check Release

1. Go to **Releases** tab
2. Find your new release
3. Download and test the ZIP file

---

## 🔧 Workflow Configuration

### Environment Variables

```yaml
env:
  CARGO_TERM_COLOR: always  # Colored output in logs
```

### Caching Strategy

We cache three things for faster builds:

1. **Cargo Registry** - Downloaded crates
2. **Cargo Index** - Crate metadata
3. **Build Target** - Compiled dependencies

**Cache Key:** Based on `Cargo.lock` hash

### GTK4 Dependencies

The workflow automatically bundles these DLLs:

- `libgtk-4-1.dll` - GTK4 core
- `libgdk-4-1.dll` - GDK (drawing)
- `libglib-2.0-0.dll` - GLib (utilities)
- `libgobject-2.0-0.dll` - GObject (type system)
- `libgio-2.0-0.dll` - GIO (I/O)
- `libcairo-2.dll` - Cairo (graphics)
- `libpango-1.0-0.dll` - Pango (text)
- `libgdk_pixbuf-2.0-0.dll` - Image loading
- Plus runtime dependencies (intl, iconv, etc.)

---

## 🐛 Troubleshooting

### CI Fails on Check

**Problem:** `cargo check` fails

**Solutions:**
- Fix compilation errors locally first
- Run `cargo check` before pushing
- Check error logs in Actions tab

### CI Fails on Clippy

**Problem:** Clippy warnings

**Solutions:**
```bash
# Run clippy locally
cargo clippy

# Fix warnings
cargo clippy --fix

# Commit fixes
git commit -am "fix: resolve clippy warnings"
```

### CI Fails on Format

**Problem:** Code not formatted

**Solutions:**
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Commit formatted code
git commit -am "style: format code"
```

### Release Build Fails

**Problem:** Release compilation fails

**Solutions:**
- Test release build locally:
  ```bash
  cargo build --release
  ```
- Check for platform-specific issues
- Review error logs in Actions

### Missing DLLs in Release

**Problem:** Application won't run (missing DLL)

**Solutions:**
- Add missing DLL to workflow
- Test on clean Windows install
- Use Dependency Walker to find missing DLLs

---

## 📊 Workflow Status Badges

Add to README.md:

```markdown
![CI](https://github.com/mg3994/lwm-gtk4-rust/workflows/CI/badge.svg)
![Release](https://github.com/mg3994/lwm-gtk4-rust/workflows/Build%20and%20Release/badge.svg)
```

---

## 🔐 Secrets

### Required Secrets

- `GITHUB_TOKEN` - Automatically provided by GitHub

### Optional Secrets

None currently required. Future additions might include:

- Code signing certificates
- Deployment credentials
- API keys for services

---

## 📈 Workflow Optimization

### Current Optimizations

1. **Caching** - Reduces build time by ~70%
2. **Parallel Jobs** - CI jobs run in parallel
3. **Conditional Steps** - Release only on tags

### Future Improvements

- [ ] Matrix builds for multiple Rust versions
- [ ] Cross-platform builds (Linux, macOS)
- [ ] Automated testing on different Windows versions
- [ ] Code coverage reporting
- [ ] Performance benchmarking

---

## 🎯 Best Practices

### Before Pushing

```bash
# Run all checks locally
cargo check
cargo test
cargo clippy
cargo fmt

# Or use a script
./scripts/pre-push.sh  # If you create one
```

### Commit Often

- Small, focused commits
- Clear commit messages
- Push to feature branches
- Create PRs for review

### Tag Wisely

- Use semantic versioning (v1.0.0)
- Create annotated tags with messages
- Only tag stable releases
- Test before tagging

---

## 📝 Workflow Files

### CI Workflow Structure

```yaml
name: CI
on: [push, pull_request]
jobs:
  check:    # Verify compilation
  test:     # Run tests
  clippy:   # Lint code
  fmt:      # Check formatting
```

### Release Workflow Structure

```yaml
name: Build and Release
on: [tags, workflow_dispatch]
jobs:
  build-windows:
    steps:
      - Setup environment
      - Build release
      - Bundle dependencies
      - Create archive
      - Upload to GitHub Releases
```

---

## 🚀 Quick Reference

### Trigger CI

```bash
git push origin main
```

### Trigger Release

```bash
git tag -a v1.0.0 -m "Release message"
git push origin v1.0.0
```

### Manual Release

1. Go to **Actions** tab
2. Select **Build and Release**
3. Click **Run workflow**
4. Choose branch
5. Click **Run workflow** button

---

## 📞 Support

If workflows fail:

1. Check **Actions** tab for logs
2. Review error messages
3. Test locally first
4. Open an issue if needed

---

**Happy Releasing! 🎉**
