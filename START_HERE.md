# 🚀 Start Here - wasm-image-processor

## Quick Setup (5 minutes)

### 1. Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli
cargo install wasm-bindgen-cli
```

### 2. Install & Build

```bash
# Install Node.js dependencies
npm install

# Build WASM module
npm run build:wasm
```

### 3. Run

```bash
# Start development server (with HMR)
npm run dev
```

Opens automatically at **http://localhost:8080** 🎉

## Available Commands

```bash
npm run dev          # Start Vite dev server with HMR
npm run build:wasm   # Build WASM module only
npm run build        # Full production build
npm run preview      # Preview production build
```

## Project Features

✅ **Resize** - Maintain aspect ratio (Lanczos3)
✅ **Blur** - Gaussian blur effect
✅ **Brightness** - Adjust image brightness
✅ **Grayscale** - Convert to grayscale
✅ **Download** - Export as PNG
✅ **Performance** - 3-4× faster than Canvas API

## Tech Stack

- **Rust** - Image processing in WebAssembly
- **Vite** - Modern build tool with HMR
- **Vanilla JS** - No framework, pure performance
- **wasm-bindgen** - Rust/JS interop

## Documentation

- **[README.md](README.md)** - Complete documentation
- **[QUICKSTART.md](QUICKSTART.md)** - Detailed setup guide
- **[EXAMPLES.md](EXAMPLES.md)** - Code examples
- **[TECH_STACK.md](TECH_STACK.md)** - Technology details
- **[DEPLOYMENT.md](DEPLOYMENT.md)** - Deployment options

## Need Help?

- Check existing documentation
- Open an issue on GitHub
- Read the blog post: [joanleon.dev](https://joanleon.dev/posts/webassembly-rust-optimizacion-imagenes/)

---

**Ready to start?** Run `npm run dev` and open http://localhost:8080
