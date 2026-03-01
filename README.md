# 🦀 WebAssembly Image Processor

High-performance image processing using Rust and WebAssembly. This project demonstrates how to leverage Rust's `image` crate compiled to WASM for blazing-fast image manipulation in the browser.

## ✨ Features

- **🔄 Resize**: Maintain aspect ratio with Lanczos3 filtering
- **✨ Blur**: Gaussian blur with configurable sigma
- **☀️ Brightness**: Adjust brightness levels (-100 to +100)
- **⚫ Grayscale**: Luminance-based conversion
- **💾 Export**: Save as PNG or JPEG with quality control
- **⚡ Performance**: 3-4× faster than Canvas API

## 🚀 Performance Benchmarks

Comparison between WebAssembly (Rust) and Canvas API on 2000×1500px images:

| Operation | Canvas API | WebAssembly | Improvement |
|-----------|------------|-------------|-------------|
| Resize    | ~45ms      | ~12ms       | **3.75× faster** |
| Gaussian Blur | ~120ms | ~28ms       | **4.3× faster** |
| Brightness | ~35ms     | ~8ms        | **4.4× faster** |
| Grayscale | ~30ms      | ~7ms        | **4.3× faster** |

*Results may vary based on hardware and browser*

## 📋 Prerequisites

- **Rust** (1.70+): [Install Rust](https://rustup.rs/)
- **wasm-bindgen-cli**: Install with `cargo install wasm-bindgen-cli`
- **Node.js** (18+): [Install Node.js](https://nodejs.org/)
- **wasm32 target**: `rustup target add wasm32-unknown-unknown`

## 🛠️ Setup & Installation

### 1. Clone the repository

```bash
git clone https://github.com/nucliweb/wasm-image-processor.git
cd wasm-image-processor
```

### 2. Build the WebAssembly module

```bash
npm run build:wasm
```

This command:
- Compiles Rust code to WebAssembly
- Generates JavaScript bindings with wasm-bindgen
- Creates optimized WASM binary in `pkg/` directory

### 3. Install Node.js dependencies

```bash
npm install
```

### 4. Start the development server

```bash
npm run dev
```

Vite will automatically:
- Start the development server at `http://localhost:8080`
- Open your browser
- Enable Hot Module Replacement (HMR)

## 📁 Project Structure

```
wasm-image-processor/
├── Cargo.toml           # Rust dependencies and build config
├── vite.config.js       # Vite configuration
├── index.html           # Entry point HTML
├── src/
│   ├── lib.rs           # Rust image processing implementation
│   ├── main.js          # JavaScript integration
│   └── style.css        # Styling
├── pkg/                 # Generated WASM output (after build)
├── dist/                # Production build output
├── package.json         # Node.js config and scripts
└── README.md
```

## 🎯 Usage

1. **Upload an image**: Click "Choose Image" and select a PNG, JPEG, or WebP file
2. **Apply operations**: Use the control buttons to process the image
3. **View results**: See original and processed images side-by-side
4. **Download**: Save the processed image as PNG

## 🔧 Development

### Build for production

```bash
npm run build        # Builds WASM and creates production bundle
npm run preview      # Preview production build locally
```

### Optimization settings

The project uses aggressive optimization in `Cargo.toml`:

```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit for better optimization
```

### Key Rust features

- **Error handling**: All operations return `Result<T, JsValue>`
- **Panic hooks**: `console_error_panic_hook` for browser debugging
- **Format support**: PNG, JPEG, and WebP via `image` crate
- **Filter quality**: Lanczos3 for high-quality downscaling

## 📚 API Reference

### ImageProcessor

```rust
// Create processor from image bytes
let processor = ImageProcessor::new(data)?;

// Resize maintaining aspect ratio
processor.resize(800, 600)?;

// Apply Gaussian blur
processor.blur(2.0)?;

// Adjust brightness (-100 to 100)
processor.brightness(20)?;

// Convert to grayscale
processor.grayscale()?;

// Export as PNG
let png_bytes = processor.to_png()?;

// Export as JPEG with quality (1-100)
let jpeg_bytes = processor.to_jpeg(85)?;

// Get dimensions
let width = processor.width();
let height = processor.height();
```

## 🌐 Browser Support

- Chrome/Edge 90+
- Firefox 89+
- Safari 15+

WebAssembly is supported in all modern browsers.

## 📖 Learn More

Read the full blog post explaining the implementation and optimization techniques:

**[WebAssembly + Rust: Optimización de imágenes](https://joanleon.dev/posts/webassembly-rust-optimizacion-imagenes/)**

Topics covered:
- Why Rust + WebAssembly for image processing
- Performance comparison methodology
- Memory management between JS and WASM
- Real-world optimization techniques

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

## 👤 Author

**Joan Leon**
- Website: [joanleon.dev](https://joanleon.dev)
- Twitter: [@nucliweb](https://twitter.com/nucliweb)
- GitHub: [@nucliweb](https://github.com/nucliweb)

## 🙏 Acknowledgments

- [image crate](https://github.com/image-rs/image) - Rust image processing library
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust/WASM/JS interop
- [Vite](https://vitejs.dev/) - Modern build tool with native WASM support

---

⭐ If you find this project useful, please consider giving it a star on GitHub!
