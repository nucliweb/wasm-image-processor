# Quick Start Guide

## First Time Setup (5 minutes)

### 1. Install Prerequisites

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli
cargo install wasm-bindgen-cli

# Verify installations
rustc --version
cargo --version
wasm-bindgen --version
node --version
```

### 2. Build and Run

```bash
# Install Node.js dependencies
npm install

# Build the WASM module
npm run build:wasm

# Start the development server (with HMR)
npm run dev
```

Vite will automatically open your browser at `http://localhost:8080`

## Development Workflow

### After making Rust changes

```bash
npm run build:wasm  # Rebuild WASM module
# Vite will auto-reload
```

### After making frontend changes (HTML/CSS/JS)

Vite HMR updates automatically - no manual refresh needed!

## Common Issues

### Build fails with "wasm-pack not found"

```bash
cargo install wasm-pack
```

### Server doesn't start

```bash
npm install  # Install http-server
npm run serve
```

### Browser shows "Cannot find module pkg/wasm_image_processor.js"

```bash
npm run build:wasm  # Generate WASM module first
```

## Project Commands

| Command | Description |
|---------|-------------|
| `npm run build:wasm` | Compile Rust to WASM |
| `npm run dev` | Start Vite dev server (HMR enabled) |
| `npm run build` | Production build (WASM + bundle) |
| `npm run preview` | Preview production build |
| `cargo test` | Run Rust unit tests |
| `cargo clippy` | Lint Rust code |

## File Editing Guide

- **src/lib.rs** - Rust image processing logic
- **src/main.js** - JavaScript integration code
- **index.html** - UI structure (project root)
- **src/style.css** - Styling
- **Cargo.toml** - Rust dependencies
- **package.json** - Node.js scripts
- **vite.config.js** - Vite configuration

## Next Steps

1. Try uploading an image
2. Apply different operations
3. Check the performance stats
4. Read the full blog post: [joanleon.dev/posts/webassembly-rust-optimizacion-imagenes](https://joanleon.dev/posts/webassembly-rust-optimizacion-imagenes/)

## Need Help?

- Check the [README.md](README.md) for detailed documentation
- Review [src/lib.rs](src/lib.rs) for code examples
- Open an issue on GitHub
