# Technology Stack

## Overview

This project uses a modern, actively maintained stack for building high-performance WebAssembly applications.

## Core Technologies

### Rust + WebAssembly

- **Rust**: 1.70+ (2021 edition)
- **Target**: `wasm32-unknown-unknown`
- **Bindgen**: `wasm-bindgen` 0.2.114
- **Image Processing**: `image` crate 0.25

**Why Rust for WASM?**
- Zero-cost abstractions
- Memory safety without garbage collection
- Predictable performance
- Rich ecosystem (crates.io)
- First-class WASM support

### Build Tooling

**Vite** 5.x
- Lightning-fast dev server with HMR
- Native ES modules support
- Built-in WASM support via plugins
- Optimized production builds
- Active maintenance

**Plugins Used:**
- `vite-plugin-wasm` - Seamless WASM integration
- `vite-plugin-top-level-await` - Enables top-level await for WASM init

**Why Vite over wasm-pack?**
- wasm-pack is no longer maintained (sunset July 2025)
- Vite is actively developed and widely used
- Better integration with modern JavaScript tooling
- Hot Module Replacement (HMR)
- Faster development experience

### Frontend

- **HTML5**: Semantic, accessible markup
- **CSS3**: Modern features (Grid, Custom Properties)
- **Vanilla JavaScript**: ES2020+ (modules, async/await)
- **No framework**: Keeps bundle small, performance predictable

## Dependencies

### Rust Dependencies (Cargo.toml)

```toml
[dependencies]
wasm-bindgen = "0.2"              # JS/WASM interop
image = { version = "0.25",
  default-features = false,
  features = ["png", "jpeg", "webp"] }
console_error_panic_hook = "0.1"  # Better error messages in browser

[dev-dependencies]
wasm-bindgen-test = "0.3"         # WASM testing
```

### Node.js Dependencies (package.json)

```json
{
  "devDependencies": {
    "vite": "^5.4.11",
    "vite-plugin-wasm": "^3.3.0",
    "vite-plugin-top-level-await": "^1.4.4"
  }
}
```

## Development Tools

### Required

- **Rust**: `rustup` toolchain manager
- **wasm-bindgen-cli**: `cargo install wasm-bindgen-cli`
- **wasm32 target**: `rustup target add wasm32-unknown-unknown`
- **Node.js**: 18+ with npm

### Optional (Recommended)

- **rust-analyzer**: LSP for IDE support
- **cargo-watch**: Auto-rebuild on file changes
- **wasm-opt**: Further optimize WASM binaries (from Binaryen)

```bash
# Install optional tools
rustup component add rust-analyzer
cargo install cargo-watch
# wasm-opt is included with Binaryen
brew install binaryen  # macOS
```

## Architecture

### Build Pipeline

```
┌─────────────┐
│  Rust Code  │
│  (src/*.rs) │
└──────┬──────┘
       │
       ▼
┌─────────────────────┐
│ cargo build         │
│ --target wasm32     │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ wasm-bindgen        │
│ (generate bindings) │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ WASM + JS Bindings  │
│ (pkg/ directory)    │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Vite (bundler)      │
│ + plugins           │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Optimized Bundle    │
│ (dist/ directory)   │
└─────────────────────┘
```

### Runtime Architecture

```
Browser
  │
  ├─ HTML (index.html)
  │
  ├─ CSS (src/style.css)
  │
  ├─ JavaScript (src/main.js)
  │    │
  │    ├─ Import WASM module
  │    ├─ Initialize ImageProcessor
  │    └─ Handle UI interactions
  │
  └─ WebAssembly (pkg/*.wasm)
       │
       ├─ ImageProcessor class
       ├─ Image operations
       └─ Memory management
```

## Performance Characteristics

### Build Time

- **Rust compilation**: ~5-10s (first build), <1s (incremental)
- **wasm-bindgen**: ~1s
- **Vite bundling**: ~1-2s (dev), ~3-5s (production)

### Bundle Size

- **WASM binary**: ~800KB (uncompressed), ~300KB (gzipped)
- **JS bindings**: ~13KB
- **Frontend code**: ~20KB (HTML + CSS + JS)
- **Total**: ~333KB gzipped (first load)

### Runtime Performance

- **WASM init**: ~10-50ms
- **Image load**: ~10-50ms (depends on size)
- **Operations**: 5-40ms (3-4× faster than Canvas API)

## Browser Compatibility

### Requirements

- **WebAssembly**: All modern browsers (Chrome 57+, Firefox 52+, Safari 11+, Edge 16+)
- **ES Modules**: Native support (Chrome 61+, Firefox 60+, Safari 11+)
- **Canvas API**: Universal support

### Tested Browsers

✅ Chrome/Edge 90+
✅ Firefox 89+
✅ Safari 15+

## Development Workflow

### Local Development

```bash
npm run dev              # Start Vite dev server (HMR)
npm run build:wasm       # Rebuild WASM only
npm run build            # Full production build
npm run preview          # Preview production build
```

### Code Quality

```bash
cargo fmt                # Format Rust code
cargo clippy             # Lint Rust code
cargo test               # Run tests
cargo bench              # Run benchmarks (if added)
```

## Deployment

Supports all major platforms:
- Vercel (recommended)
- Netlify
- GitHub Pages
- Cloudflare Pages
- Self-hosted (Docker/nginx)

See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed instructions.

## Future Considerations

### Potential Optimizations

1. **wasm-opt**: Further size reduction with Binaryen
2. **Code splitting**: Lazy load WASM for large apps
3. **Worker threads**: Off-main-thread processing
4. **Shared memory**: For multi-threaded operations

### Alternative Technologies

If requirements change:
- **AssemblyScript**: TypeScript-like syntax for WASM
- **Go + TinyGo**: Alternative systems language
- **C++ + Emscripten**: Existing C++ codebases

## References

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Vite Documentation](https://vitejs.dev/)
- [image crate docs](https://docs.rs/image/latest/image/)

---

Last updated: March 2026
