# Contributing to LinkWithMentor

Thank you for your interest in contributing to LinkWithMentor! 🎉

## 🚀 Getting Started

### Prerequisites

- **Rust** (latest stable) - https://rustup.rs/
- **GTK4** - See [WINDOWS_SETUP.md](WINDOWS_SETUP.md) for installation
- **Git** - For version control

### Setup Development Environment

1. **Clone the repository:**
   ```bash
   git clone https://github.com/mg3994/lwm-gtk4-rust.git
   cd lwm-gtk4-rust
   ```

2. **Install GTK4** (if not already installed):
   - Follow the detailed instructions in [WINDOWS_SETUP.md](WINDOWS_SETUP.md)

3. **Build the project:**
   ```bash
   cargo build
   ```

4. **Run the application:**
   ```bash
   cargo run
   ```

## 📝 Development Workflow

### Branch Strategy

- `main` - Stable, production-ready code
- `develop` - Integration branch for features
- `feature/*` - New features
- `fix/*` - Bug fixes
- `docs/*` - Documentation updates

### Making Changes

1. **Create a new branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the code style

3. **Test your changes:**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

4. **Commit your changes:**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

5. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request** on GitHub

### Commit Message Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

**Examples:**
```
feat: add analytics dashboard screen
fix: resolve navigation button click issue
docs: update installation instructions
style: format code with rustfmt
```

## 🎨 Code Style

### Rust Code

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Add comments for complex logic
- Keep functions focused and small

### CSS

- Use consistent naming (kebab-case for classes)
- Group related styles together
- Add comments for complex styling
- Follow the existing color scheme

### File Organization

```
src/
├── main.rs          - Main application and view functions
resources/
├── style.css        - All CSS styling
├── ui.glade         - XML UI definitions (optional)
.github/
├── workflows/       - CI/CD workflows
docs/
├── *.md            - Documentation files
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Adding Tests

- Add unit tests in the same file as the code
- Add integration tests in `tests/` directory
- Test edge cases and error conditions

## 📚 Documentation

### Code Documentation

- Add doc comments (`///`) for public functions
- Include examples in doc comments
- Document parameters and return values

**Example:**
```rust
/// Creates a new post card widget
///
/// # Arguments
/// * `user` - The username of the post author
/// * `content` - The post content text
/// * `time` - Timestamp string (e.g., "2 hours ago")
/// * `emoji` - Emoji avatar for the user
///
/// # Returns
/// A `Frame` widget containing the formatted post card
fn create_post_card(user: &str, content: &str, time: &str, emoji: &str) -> Frame {
    // Implementation
}
```

### User Documentation

- Update README.md for user-facing changes
- Update CODE_GUIDE.md for developer documentation
- Add screenshots for UI changes

## 🐛 Reporting Bugs

### Before Reporting

1. Check if the bug has already been reported
2. Try to reproduce the bug
3. Gather relevant information

### Bug Report Template

```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '...'
3. See error

**Expected behavior**
What you expected to happen.

**Screenshots**
If applicable, add screenshots.

**Environment:**
 - OS: [e.g., Windows 11]
 - GTK4 Version: [e.g., 4.12.0]
 - Rust Version: [e.g., 1.75.0]

**Additional context**
Any other context about the problem.
```

## ✨ Feature Requests

### Feature Request Template

```markdown
**Is your feature request related to a problem?**
A clear description of the problem.

**Describe the solution you'd like**
A clear description of what you want to happen.

**Describe alternatives you've considered**
Alternative solutions or features you've considered.

**Additional context**
Any other context or screenshots.
```

## 🔄 Pull Request Process

1. **Update documentation** if needed
2. **Add tests** for new features
3. **Ensure CI passes** (all checks green)
4. **Request review** from maintainers
5. **Address feedback** promptly
6. **Squash commits** if requested

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Tests added/updated
- [ ] Manual testing completed
- [ ] CI passes

## Screenshots (if applicable)
Add screenshots of UI changes

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings
```

## 🎯 Areas for Contribution

### High Priority

- [ ] Implement remaining 6 screens (Analytics, Events, Jobs, Achievements, Learning, Discover)
- [ ] Add backend API integration
- [ ] Implement real WebRTC for video calls
- [ ] Add database support
- [ ] User authentication

### Medium Priority

- [ ] Improve error handling
- [ ] Add more tests
- [ ] Performance optimizations
- [ ] Accessibility improvements
- [ ] Internationalization (i18n)

### Good First Issues

- [ ] Fix typos in documentation
- [ ] Add more CSS animations
- [ ] Improve existing UI components
- [ ] Add code comments
- [ ] Update README examples

## 📞 Getting Help

- **GitHub Issues** - For bugs and feature requests
- **Discussions** - For questions and ideas
- **Discord** - [Link if available]

## 📜 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🙏 Thank You!

Your contributions make LinkWithMentor better for everyone. We appreciate your time and effort!

---

**Happy Coding! 🚀**
