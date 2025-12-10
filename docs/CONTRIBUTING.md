# Contributing to Stardust

Thank you for your interest in contributing to Stardust! This document provides guidelines for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/stardust.git
   cd stardust
   ```
3. **Set up the development environment**:
   ```bash
   cargo build
   ```

## Development Workflow

1. **Create a new branch** for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the code style guidelines

3. **Test your changes**:
   ```bash
   cargo test
   cargo check
   cargo clippy
   ```

4. **Commit your changes** with a descriptive message:
   ```bash
   git commit -m "Add feature: description of your feature"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request** on GitHub

## Code Style

- Follow Rust's standard formatting with `rustfmt`:
  ```bash
  cargo fmt
  ```

- Address any warnings from Clippy:
  ```bash
  cargo clippy
  ```

- Write clear, concise comments for complex logic

- Keep functions small and focused on a single task

## Commit Messages

- Use present tense ("Add feature" not "Added feature")
- Start with a capital letter
- Keep the first line under 50 characters
- Provide detailed description if needed after a blank line

## Testing

- Add tests for new features when applicable
- Ensure all tests pass before submitting a PR
- Test on multiple platforms if possible (Linux, macOS, Windows)

## Areas for Contribution

### High Priority
- Application data models and storage
- Package manager integration
- Application list rendering
- Download and installation functionality

### Medium Priority
- Theme customization
- Search improvements (fuzzy matching, filters)
- Application details view
- Settings page

### Nice to Have
- Internationalization (i18n)
- Accessibility improvements
- Performance optimizations
- Documentation improvements

## Questions?

If you have questions about contributing, feel free to:
- Open an issue on GitHub
- Check existing issues and discussions
- Review the [DESIGN.md](DESIGN.md) document

## License

By contributing to Stardust, you agree that your contributions will be licensed under the GNU General Public License v3.0.
