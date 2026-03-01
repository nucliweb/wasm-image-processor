# Usage Examples

## Basic Image Processing

### Load and Display an Image

```javascript
import init, { ImageProcessor } from './pkg/wasm_image_processor.js';

// Initialize WASM module
await init();

// Load image from file input
const fileInput = document.getElementById('file-input');
fileInput.addEventListener('change', async (e) => {
    const file = e.target.files[0];
    const arrayBuffer = await file.arrayBuffer();
    const imageData = new Uint8Array(arrayBuffer);

    // Create processor
    const processor = new ImageProcessor(imageData);
    console.log(`Image loaded: ${processor.width()}×${processor.height()}px`);
});
```

### Resize Image

```javascript
// Resize to fit within 800×600 (maintains aspect ratio)
processor.resize(800, 600);

// Export as PNG
const pngData = processor.to_png();
displayImage(pngData);
```

### Apply Blur Effect

```javascript
// Light blur
processor.blur(1.0);

// Medium blur
processor.blur(3.0);

// Strong blur
processor.blur(5.0);
```

### Adjust Brightness

```javascript
// Make darker
processor.brightness(-30);

// Make brighter
processor.brightness(30);

// Reset (no change)
processor.brightness(0);
```

### Convert to Grayscale

```javascript
processor.grayscale();
```

## Advanced Examples

### Processing Pipeline

```javascript
// Apply multiple operations in sequence
const processor = new ImageProcessor(imageData);

processor.resize(1920, 1080);  // Resize first
processor.brightness(10);       // Then adjust brightness
processor.blur(0.5);            // Add subtle blur

const result = processor.to_jpeg(90);  // Export as JPEG
```

### Error Handling

```javascript
try {
    const processor = new ImageProcessor(imageData);
    processor.brightness(150);  // Invalid: > 100
} catch (error) {
    console.error('Processing failed:', error);
}
```

### Performance Monitoring

```javascript
const startTime = performance.now();

const processor = new ImageProcessor(imageData);
processor.resize(800, 600);
const result = processor.to_png();

const duration = performance.now() - startTime;
console.log(`Processing completed in ${duration.toFixed(2)}ms`);
```

### Format Conversion

```javascript
// Load any supported format (PNG, JPEG, WebP)
const processor = new ImageProcessor(imageData);

// Export as PNG (lossless)
const pngData = processor.to_png();

// Export as JPEG with quality control
const jpegHighQuality = processor.to_jpeg(95);
const jpegMediumQuality = processor.to_jpeg(80);
const jpegLowQuality = processor.to_jpeg(60);
```

### Download Processed Image

```javascript
function downloadImage(imageData, filename) {
    const blob = new Blob([imageData], { type: 'image/png' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = filename;
    link.click();
    URL.revokeObjectURL(url);
}

// Usage
const processor = new ImageProcessor(imageData);
processor.grayscale();
const result = processor.to_png();
downloadImage(result, 'grayscale.png');
```

### Batch Processing

```javascript
async function processBatch(files) {
    await init();  // Initialize once

    for (const file of files) {
        const arrayBuffer = await file.arrayBuffer();
        const imageData = new Uint8Array(arrayBuffer);

        const processor = new ImageProcessor(imageData);
        processor.resize(800, 600);
        processor.grayscale();

        const result = processor.to_jpeg(85);
        downloadImage(result, `processed_${file.name}`);
    }
}
```

### Responsive Thumbnails

```javascript
// Create multiple thumbnail sizes
async function createThumbnails(imageData) {
    const sizes = [
        { width: 150, height: 150, name: 'small' },
        { width: 300, height: 300, name: 'medium' },
        { width: 600, height: 600, name: 'large' }
    ];

    const thumbnails = {};

    for (const size of sizes) {
        const processor = new ImageProcessor(imageData);
        processor.resize(size.width, size.height);
        thumbnails[size.name] = processor.to_jpeg(85);
    }

    return thumbnails;
}
```

### Image Comparison

```javascript
// Compare processing methods
async function compareProcessing(imageData) {
    const startWasm = performance.now();
    const processor = new ImageProcessor(imageData);
    processor.blur(2.0);
    const wasmResult = processor.to_png();
    const wasmTime = performance.now() - startWasm;

    // Compare with Canvas API
    const startCanvas = performance.now();
    const canvasResult = await blurWithCanvas(imageData, 2.0);
    const canvasTime = performance.now() - startCanvas;

    console.log(`WASM: ${wasmTime.toFixed(2)}ms`);
    console.log(`Canvas: ${canvasTime.toFixed(2)}ms`);
    console.log(`Speedup: ${(canvasTime / wasmTime).toFixed(2)}×`);
}
```

## Rust API Examples

### Creating Custom Operations

```rust
use wasm_bindgen::prelude::*;
use image::DynamicImage;

#[wasm_bindgen]
impl ImageProcessor {
    /// Custom operation: Apply sepia tone
    #[wasm_bindgen]
    pub fn sepia(&mut self) -> Result<(), JsValue> {
        // Convert to grayscale first
        let gray = self.image.grayscale();

        // Apply sepia tone transformation
        let rgb = gray.to_rgb8();
        // ... sepia transformation logic ...

        Ok(())
    }
}
```

### Validation and Error Handling

```rust
#[wasm_bindgen]
pub fn resize(&mut self, max_width: u32, max_height: u32) -> Result<(), JsValue> {
    if max_width == 0 || max_height == 0 {
        return Err(JsValue::from_str("Dimensions must be greater than 0"));
    }

    // ... resize logic ...

    Ok(())
}
```

## Tips and Best Practices

1. **Initialize Once**: Call `init()` once at app startup
2. **Memory Management**: Process images one at a time for large files
3. **Quality vs Size**: Use JPEG quality 80-85 for optimal balance
4. **Error Handling**: Always wrap operations in try-catch blocks
5. **Performance**: Monitor processing times for large images
6. **Format Selection**:
   - PNG: Lossless, larger files
   - JPEG: Lossy, smaller files (photos)
   - WebP: Best compression (modern browsers)

## Common Patterns

### Photo Editor Pattern

```javascript
class PhotoEditor {
    constructor() {
        this.original = null;
        this.history = [];
    }

    async load(imageData) {
        this.original = imageData;
        this.processor = new ImageProcessor(imageData);
    }

    apply(operation) {
        this.history.push(this.processor.to_png());
        operation(this.processor);
    }

    undo() {
        if (this.history.length > 0) {
            const previous = this.history.pop();
            this.processor = new ImageProcessor(previous);
        }
    }

    reset() {
        this.processor = new ImageProcessor(this.original);
        this.history = [];
    }
}
```

### Filter Chain Pattern

```javascript
class FilterChain {
    constructor(imageData) {
        this.filters = [];
        this.imageData = imageData;
    }

    resize(w, h) {
        this.filters.push(p => p.resize(w, h));
        return this;
    }

    blur(sigma) {
        this.filters.push(p => p.blur(sigma));
        return this;
    }

    async execute() {
        const processor = new ImageProcessor(this.imageData);
        for (const filter of this.filters) {
            filter(processor);
        }
        return processor.to_png();
    }
}

// Usage
const result = await new FilterChain(imageData)
    .resize(800, 600)
    .brightness(10)
    .blur(1.0)
    .execute();
```
