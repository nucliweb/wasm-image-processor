# 🧪 Test Suite

Comprehensive test coverage for the WASM Image Processor.

## 📊 Test Coverage

### Files Tested

#### JavaScript/TypeScript Tests
- **`benchmark.test.js`** (15 tests) - BenchmarkSuite functionality
- **`canvas-processor.test.js`** (23 tests) - Canvas API implementation
- **Total JavaScript:** 38 tests

#### Rust Tests
- **`src/lib.rs`** (22 tests) - Native unit tests for image processing logic
- **`tests/web.rs`** (13 tests) - WASM integration tests in browser
- **Total Rust:** 35 tests

**Grand Total:** 73 tests

## 🚀 Running Tests

### JavaScript Tests
```bash
# Run all JavaScript tests
npm test

# Run tests in watch mode
npm test

# Run tests once
npm run test:run

# Run tests with UI
npm run test:ui
```

### Rust Tests
```bash
# Run native unit tests (fast)
cargo test --lib

# Run WASM integration tests in browser
wasm-pack test --headless --chrome

# Or with Firefox
wasm-pack test --headless --firefox

# Run all Rust tests
cargo test --lib && wasm-pack test --headless --chrome
```

## 🏗️ Test Infrastructure

### Synthetic Image Generation

Tests use **synthetic images** generated programmatically, no external image files required:

- **`generateTestImage(width, height)`** - Gradient pattern with shapes
- **`generateSolidColorImage(width, height, color)`** - Solid color
- **`generateCheckerboardImage(width, height, squareSize)`** - Checkerboard pattern

### Environment

- **Test Runner:** Vitest
- **Environment:** jsdom
- **Canvas Implementation:** node-canvas
- **WASM Loader:** Custom setup with file-based loading

## 📝 Test Categories

### Rust Unit Tests (src/lib.rs)

#### Image Loading
- ✅ Loads valid image data
- ✅ Fails with invalid/corrupted data

#### Resize Operation
- ✅ Maintains aspect ratio
- ✅ Does not upscale images
- ✅ Updates dimensions correctly

#### Blur Operation
- ✅ Rejects zero sigma
- ✅ Rejects negative sigma
- ✅ Accepts valid sigma values

#### Brightness Operation
- ✅ Rejects values below -100
- ✅ Rejects values above 100
- ✅ Accepts minimum value (-100)
- ✅ Accepts maximum value (100)
- ✅ Accepts zero (no change)

#### Grayscale Operation
- ✅ Processes image successfully

#### PNG Export
- ✅ Exports valid PNG data
- ✅ Validates PNG magic number (89 50 4E 47)

#### JPEG Export
- ✅ Rejects quality of 0
- ✅ Rejects quality above 100
- ✅ Accepts minimum quality (1)
- ✅ Accepts maximum quality (100)
- ✅ Accepts recommended quality (85)
- ✅ Validates JPEG magic number (FF D8 FF)

#### Dimensions
- ✅ Returns correct width
- ✅ Returns correct height
- ✅ Updates after resize operation

### Rust WASM Integration Tests (tests/web.rs)

#### WASM Interface
- ✅ Creates processor from valid image
- ✅ Fails with invalid image data
- ✅ Validates parameters through WASM boundary
- ✅ Chains multiple operations
- ✅ Exports to PNG/JPEG formats
- ✅ Returns accurate dimensions

### BenchmarkSuite Tests

#### Benchmark Infrastructure
- ✅ Creates BenchmarkSuite instance
- ✅ Has correct initial results structure

#### Individual Benchmarks
- ✅ Benchmarks resize operation
- ✅ Benchmarks blur operation
- ✅ Benchmarks brightness operation
- ✅ Benchmarks grayscale operation

#### Full Benchmark Suite
- ✅ Runs all benchmarks with progress callback
- ✅ WASM is faster than Canvas API for most operations

#### Utility Methods
- ✅ Calculates median correctly
- ✅ Formats time correctly
- ✅ Formats improvement correctly

#### Performance Characteristics
- ✅ Multiple runs produce consistent results
- ✅ Benchmarks produce valid timing data

#### Edge Cases
- ✅ Handles very small images (10×10px)
- ✅ Handles rectangular images

### CanvasProcessor Tests

#### Initialization
- ✅ Creates CanvasProcessor instance
- ✅ Loads image data correctly

#### Resize Operation
- ✅ Resizes image to specified dimensions
- ✅ Resizes to larger dimensions
- ✅ Produces valid PNG output after resize

#### Blur Operation
- ✅ Applies blur without errors
- ✅ Maintains image dimensions after blur
- ✅ Produces valid output after blur

#### Brightness Operation
- ✅ Increases brightness
- ✅ Decreases brightness
- ✅ Maintains dimensions after brightness adjustment
- ✅ Clamps brightness values correctly

#### Grayscale Operation
- ✅ Converts image to grayscale
- ✅ Maintains dimensions after grayscale conversion
- ✅ Produces valid output after grayscale

#### PNG Export
- ✅ Exports to PNG format (validates signature)
- ✅ Exports correct dimensions

#### Operation Chaining
- ✅ Chains multiple operations sequentially

#### Edge Cases
- ✅ Handles very small images
- ✅ Handles rectangular images
- ✅ Cleans up resources properly

#### Box Blur Algorithm
- ✅ Box blur produces output
- ✅ Box blur with zero radius returns similar data

## 🔧 Test Utilities

### `test-utils.js`

Provides helper functions for testing:

```javascript
// Generate test images
const testImage = await generateTestImage(100, 100);

// Get image dimensions
const { width, height } = await getImageDimensions(imageData);

// Compare images (similarity percentage)
const similarity = await compareImages(image1, image2);
```

### `setup.js`

- Initializes WASM module from filesystem
- Polyfills Canvas API for node.js environment
- Runs once before all tests

## 📈 Performance Expectations

The tests verify:

- **Timing validity**: All operations complete in 0.1ms - 10s
- **Consistency**: Multiple runs within 2× variance
- **Correctness**: All operations produce valid PNG output
- **Dimensions**: Sizes are preserved or changed as expected

## 🐛 Debugging Tests

```bash
# Run specific test file
npm test tests/benchmark.test.js

# Run tests matching pattern
npm test -- -t "resize"

# Show verbose output
npm test -- --reporter=verbose

# Run with debugging
node --inspect-brk ./node_modules/.bin/vitest --run
```

## 📦 Dependencies

- `vitest` - Fast unit test framework
- `jsdom` - JavaScript implementation of web standards
- `canvas` - Cairo-backed Canvas implementation for Node.js

## 🎯 CI/CD Integration

These tests are designed to run in CI environments:

```yaml
- name: Run Rust Tests
  run: |
    cargo test --lib
    wasm-pack test --headless --chrome

- name: Run JavaScript Tests
  run: |
    npm run build:wasm
    npm test
```

**Note:** WASM must be built before running JavaScript tests.

## 💡 Writing New Tests

Example test structure:

```javascript
import { describe, it, expect, beforeAll } from 'vitest';
import { generateTestImage } from './test-utils.js';

describe('MyFeature', () => {
  let testImage;

  beforeAll(async () => {
    testImage = await generateTestImage(100, 100);
  });

  it('does something', async () => {
    // Your test here
    expect(result).toBe(expected);
  });
});
```

## ✅ Test Quality Guidelines

- **No external dependencies**: Use synthetic images
- **Fast execution**: Tests complete in < 2 seconds
- **Deterministic**: Same input always produces same output
- **Isolated**: Each test is independent
- **Clear assertions**: Descriptive expectations

---

**Test Coverage Goal:** Ensure benchmarking system works correctly across all operations and produces valid performance comparisons.
