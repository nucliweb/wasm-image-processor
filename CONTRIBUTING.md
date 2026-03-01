# Contributing to WebAssembly Image Processor

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/wasm-image-processor.git`
3. Create a feature branch: `git checkout -b feature/amazing-feature`
4. Make your changes
5. Test your changes
6. Commit with clear messages
7. Push to your fork
8. Open a Pull Request

## Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
cargo install wasm-pack

# Build project
npm run build

# Run local server
npm run serve
```

## Project Structure

- `src/lib.rs` - Core Rust implementation
- `www/` - Frontend application
- `tests/` - Test suite
- `Cargo.toml` - Rust dependencies
- `package.json` - Node.js scripts

## Coding Standards

### Rust Code

- Follow Rust naming conventions (snake_case for functions/variables)
- Add doc comments for public APIs
- Use `Result<T, JsValue>` for error handling
- Run `cargo fmt` before committing
- Run `cargo clippy` to check for issues

Example:
```rust
/// Resizes the image while maintaining aspect ratio.
///
/// # Arguments
/// * `max_width` - Maximum width constraint
/// * `max_height` - Maximum height constraint
#[wasm_bindgen]
pub fn resize(&mut self, max_width: u32, max_height: u32) -> Result<(), JsValue> {
    // Implementation
}
```

### JavaScript Code

- Use modern ES6+ syntax
- Add JSDoc comments for complex functions
- Handle errors with try-catch
- Use descriptive variable names

Example:
```javascript
/**
 * Applies an image processing operation
 * @param {string} operation - Operation name
 */
async function applyOperation(operation) {
    // Implementation
}
```

### CSS Code

- Use CSS custom properties (variables)
- Follow mobile-first approach
- Keep selectors simple and maintainable
- Add comments for complex layouts

## Testing

### Rust Tests

```bash
# Run unit tests
cargo test

# Run WASM tests
wasm-pack test --headless --firefox
```

### Manual Testing

1. Build the project: `npm run build`
2. Start server: `npm run serve`
3. Test all operations with various image formats
4. Check browser console for errors
5. Verify performance metrics

## Adding New Features

### Adding a New Image Operation

1. **Add Rust implementation** in `src/lib.rs`:

```rust
#[wasm_bindgen]
impl ImageProcessor {
    /// Your new operation description
    #[wasm_bindgen]
    pub fn your_operation(&mut self, param: i32) -> Result<(), JsValue> {
        // Validate parameters
        if param < 0 {
            return Err(JsValue::from_str("Parameter must be positive"));
        }

        // Apply operation
        // self.image = ...

        Ok(())
    }
}
```

2. **Add UI button** in `www/index.html`:

```html
<button id="your-operation-btn" class="control-btn" disabled>
    ✨ Your Operation
</button>
```

3. **Add JavaScript handler** in `www/index.js`:

```javascript
const yourOperationBtn = document.getElementById('your-operation-btn');
yourOperationBtn.addEventListener('click', () => applyOperation('your_operation'));

// Add case in applyOperation function
case 'your_operation':
    processor.your_operation(someParam);
    break;
```

4. **Update documentation**:
   - Add to README.md features list
   - Add example to EXAMPLES.md
   - Update benchmark table if applicable

5. **Test thoroughly**:
   - Test with different image sizes
   - Test edge cases
   - Verify performance

## Commit Messages

Use clear, descriptive commit messages:

- `feat: Add sepia tone filter`
- `fix: Correct brightness calculation`
- `docs: Update API documentation`
- `perf: Optimize blur algorithm`
- `refactor: Simplify error handling`
- `test: Add unit tests for resize`

## Pull Request Process

1. Update documentation if needed
2. Add tests for new features
3. Ensure all tests pass
4. Update CHANGELOG.md (if exists)
5. Fill out PR template completely
6. Link related issues

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All tests pass
- [ ] No new warnings from clippy
- [ ] Code formatted with rustfmt

## Performance Considerations

- Profile operations with `performance.now()`
- Test with large images (5000×5000px+)
- Compare WASM vs Canvas API when relevant
- Consider memory usage for operations
- Use appropriate image filters (Lanczos3 for quality)

## Documentation

- Keep README.md up to date
- Add examples to EXAMPLES.md
- Update QUICKSTART.md for setup changes
- Include inline comments for complex logic
- Update API reference for new methods

## Questions or Issues?

- Open an issue for bugs
- Open a discussion for feature ideas
- Ask questions in issues
- Check existing issues first

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be acknowledged in the project README.

Thank you for contributing! 🦀✨
