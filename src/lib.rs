use wasm_bindgen::prelude::*;
use image::{DynamicImage, GenericImageView, ImageFormat, imageops::FilterType};
use std::io::Cursor;

/// Set panic hook for better error messages in the browser console
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// ImageProcessor provides high-performance image manipulation capabilities
/// using Rust's image crate compiled to WebAssembly.
#[wasm_bindgen]
pub struct ImageProcessor {
    image: DynamicImage,
}

#[wasm_bindgen]
impl ImageProcessor {
    /// Creates a new ImageProcessor from raw image bytes.
    /// Supports PNG, JPEG, and WebP formats.
    ///
    /// # Arguments
    /// * `data` - Raw image bytes from uploaded file
    ///
    /// # Returns
    /// * `Result<ImageProcessor, JsValue>` - Processor instance or error
    #[wasm_bindgen(constructor)]
    pub fn new(data: &[u8]) -> Result<ImageProcessor, JsValue> {
        // Load image from bytes, automatically detecting format
        let image = image::load_from_memory(data)
            .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;

        Ok(ImageProcessor { image })
    }

    /// Resizes the image while maintaining aspect ratio.
    /// Uses Lanczos3 filter for high-quality downscaling.
    ///
    /// # Arguments
    /// * `max_width` - Maximum width constraint
    /// * `max_height` - Maximum height constraint
    ///
    /// # Example
    /// ```
    /// processor.resize(800, 600); // Fits image within 800x600 box
    /// ```
    #[wasm_bindgen]
    pub fn resize(&mut self, max_width: u32, max_height: u32) -> Result<(), JsValue> {
        // Calculate new dimensions maintaining aspect ratio
        let (width, height) = self.image.dimensions();
        let ratio = (width as f32 / max_width as f32)
            .max(height as f32 / max_height as f32);

        if ratio > 1.0 {
            let new_width = (width as f32 / ratio) as u32;
            let new_height = (height as f32 / ratio) as u32;

            // Lanczos3 provides best quality for downscaling
            self.image = self.image.resize(new_width, new_height, FilterType::Lanczos3);
        }

        Ok(())
    }

    /// Applies Gaussian blur to the image.
    ///
    /// # Arguments
    /// * `sigma` - Blur intensity (typical range: 0.5 to 10.0)
    ///   - Lower values: subtle blur
    ///   - Higher values: stronger blur effect
    #[wasm_bindgen]
    pub fn blur(&mut self, sigma: f32) -> Result<(), JsValue> {
        if sigma <= 0.0 {
            return Err(JsValue::from_str("Sigma must be greater than 0"));
        }

        // Apply Gaussian blur using the specified sigma value
        self.image = self.image.blur(sigma);
        Ok(())
    }

    /// Adjusts image brightness.
    ///
    /// # Arguments
    /// * `value` - Brightness adjustment (-100 to 100)
    ///   - Negative values: darker
    ///   - Positive values: brighter
    ///   - 0: no change
    #[wasm_bindgen]
    pub fn brightness(&mut self, value: i32) -> Result<(), JsValue> {
        if value < -100 || value > 100 {
            return Err(JsValue::from_str("Brightness value must be between -100 and 100"));
        }

        // Convert to image crate's expected range
        self.image = self.image.brighten(value);
        Ok(())
    }

    /// Converts the image to grayscale.
    /// Uses luminance-based conversion for natural-looking results.
    #[wasm_bindgen]
    pub fn grayscale(&mut self) -> Result<(), JsValue> {
        self.image = self.image.grayscale();
        Ok(())
    }

    /// Exports the current image as PNG format.
    ///
    /// # Returns
    /// * `Vec<u8>` - PNG-encoded image bytes suitable for download or display
    #[wasm_bindgen]
    pub fn to_png(&self) -> Result<Vec<u8>, JsValue> {
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        // Encode as PNG (lossless compression)
        self.image
            .write_to(&mut cursor, ImageFormat::Png)
            .map_err(|e| JsValue::from_str(&format!("Failed to encode PNG: {}", e)))?;

        Ok(buffer)
    }

    /// Exports the current image as JPEG format with specified quality.
    ///
    /// # Arguments
    /// * `quality` - JPEG compression quality (1-100)
    ///   - 1: maximum compression, lowest quality
    ///   - 100: minimum compression, highest quality
    ///   - Recommended: 80-90 for web use
    ///
    /// # Returns
    /// * `Vec<u8>` - JPEG-encoded image bytes
    #[wasm_bindgen]
    pub fn to_jpeg(&self, quality: u8) -> Result<Vec<u8>, JsValue> {
        if quality == 0 || quality > 100 {
            return Err(JsValue::from_str("Quality must be between 1 and 100"));
        }

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        // Convert to RGB (JPEG doesn't support alpha channel)
        let rgb_image = self.image.to_rgb8();

        // Encode as JPEG with specified quality
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, quality);
        rgb_image
            .write_with_encoder(encoder)
            .map_err(|e| JsValue::from_str(&format!("Failed to encode JPEG: {}", e)))?;

        Ok(buffer)
    }

    /// Returns the current width of the image in pixels.
    #[wasm_bindgen]
    pub fn width(&self) -> u32 {
        self.image.width()
    }

    /// Returns the current height of the image in pixels.
    #[wasm_bindgen]
    pub fn height(&self) -> u32 {
        self.image.height()
    }
}

// Internal implementation for testing - not exposed to WASM
#[cfg(not(target_arch = "wasm32"))]
impl ImageProcessor {
    /// Internal constructor for native tests
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let image = image::load_from_memory(data)
            .map_err(|e| format!("Failed to load image: {}", e))?;
        Ok(ImageProcessor { image })
    }

    /// Internal resize for native tests
    pub fn resize_internal(&mut self, max_width: u32, max_height: u32) -> Result<(), String> {
        let (width, height) = self.image.dimensions();
        let ratio = (width as f32 / max_width as f32)
            .max(height as f32 / max_height as f32);

        if ratio > 1.0 {
            let new_width = (width as f32 / ratio) as u32;
            let new_height = (height as f32 / ratio) as u32;
            self.image = self.image.resize(new_width, new_height, FilterType::Lanczos3);
        }
        Ok(())
    }

    /// Internal blur for native tests
    pub fn blur_internal(&mut self, sigma: f32) -> Result<(), String> {
        if sigma <= 0.0 {
            return Err("Sigma must be greater than 0".to_string());
        }
        self.image = self.image.blur(sigma);
        Ok(())
    }

    /// Internal brightness for native tests
    pub fn brightness_internal(&mut self, value: i32) -> Result<(), String> {
        if value < -100 || value > 100 {
            return Err("Brightness value must be between -100 and 100".to_string());
        }
        self.image = self.image.brighten(value);
        Ok(())
    }

    /// Internal grayscale for native tests
    pub fn grayscale_internal(&mut self) -> Result<(), String> {
        self.image = self.image.grayscale();
        Ok(())
    }

    /// Internal to_png for native tests
    pub fn to_png_internal(&self) -> Result<Vec<u8>, String> {
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        self.image
            .write_to(&mut cursor, ImageFormat::Png)
            .map_err(|e| format!("Failed to encode PNG: {}", e))?;
        Ok(buffer)
    }

    /// Internal to_jpeg for native tests
    pub fn to_jpeg_internal(&self, quality: u8) -> Result<Vec<u8>, String> {
        if quality == 0 || quality > 100 {
            return Err("Quality must be between 1 and 100".to_string());
        }

        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        let rgb_image = self.image.to_rgb8();
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, quality);
        rgb_image
            .write_with_encoder(encoder)
            .map_err(|e| format!("Failed to encode JPEG: {}", e))?;
        Ok(buffer)
    }

    /// Internal width getter
    pub fn get_width(&self) -> u32 {
        self.image.width()
    }

    /// Internal height getter
    pub fn get_height(&self) -> u32 {
        self.image.height()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    /// Creates a simple 10x10 red test image
    fn create_test_image() -> Vec<u8> {
        let img = ImageBuffer::from_fn(10, 10, |_, _| Rgb([255u8, 0u8, 0u8]));
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        DynamicImage::ImageRgb8(img)
            .write_to(&mut cursor, ImageFormat::Png)
            .unwrap();
        buffer
    }

    /// Creates an invalid image data
    fn create_invalid_image() -> Vec<u8> {
        vec![0, 1, 2, 3, 4, 5]
    }

    #[test]
    fn new_loads_valid_image() {
        let image_data = create_test_image();
        let processor = ImageProcessor::from_bytes(&image_data);

        assert!(processor.is_ok());
        let processor = processor.unwrap();
        assert_eq!(processor.get_width(), 10);
        assert_eq!(processor.get_height(), 10);
    }

    #[test]
    fn new_fails_with_invalid_data() {
        let invalid_data = create_invalid_image();
        let result = ImageProcessor::from_bytes(&invalid_data);

        assert!(result.is_err());
    }

    #[test]
    fn resize_maintains_aspect_ratio() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        processor.resize_internal(5, 5).unwrap();

        assert_eq!(processor.get_width(), 5);
        assert_eq!(processor.get_height(), 5);
    }

    #[test]
    fn resize_does_not_upscale() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        processor.resize_internal(100, 100).unwrap();

        // Should remain 10x10, not upscale to 100x100
        assert_eq!(processor.get_width(), 10);
        assert_eq!(processor.get_height(), 10);
    }

    #[test]
    fn blur_rejects_zero_sigma() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.blur_internal(0.0);

        assert!(result.is_err());
    }

    #[test]
    fn blur_rejects_negative_sigma() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.blur_internal(-1.0);

        assert!(result.is_err());
    }

    #[test]
    fn blur_accepts_valid_sigma() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.blur_internal(2.5);

        assert!(result.is_ok());
    }

    #[test]
    fn brightness_rejects_value_below_range() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.brightness_internal(-101);

        assert!(result.is_err());
    }

    #[test]
    fn brightness_rejects_value_above_range() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.brightness_internal(101);

        assert!(result.is_err());
    }

    #[test]
    fn brightness_accepts_minimum_value() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.brightness_internal(-100);

        assert!(result.is_ok());
    }

    #[test]
    fn brightness_accepts_maximum_value() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.brightness_internal(100);

        assert!(result.is_ok());
    }

    #[test]
    fn brightness_accepts_zero() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.brightness_internal(0);

        assert!(result.is_ok());
    }

    #[test]
    fn grayscale_processes_image() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.grayscale_internal();

        assert!(result.is_ok());
    }

    #[test]
    fn to_png_exports_valid_data() {
        let image_data = create_test_image();
        let processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let png_data = processor.to_png_internal();

        assert!(png_data.is_ok());
        let data = png_data.unwrap();
        assert!(!data.is_empty());
        // PNG magic number: 89 50 4E 47
        assert_eq!(&data[0..4], &[137, 80, 78, 71]);
    }

    #[test]
    fn to_jpeg_rejects_zero_quality() {
        let image_data = create_test_image();
        let processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.to_jpeg_internal(0);

        assert!(result.is_err());
    }

    #[test]
    fn to_jpeg_rejects_quality_above_100() {
        let image_data = create_test_image();
        let processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.to_jpeg_internal(101);

        assert!(result.is_err());
    }

    #[test]
    fn to_jpeg_accepts_minimum_quality() {
        let image_data = create_test_image();
        let processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.to_jpeg_internal(1);

        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(!data.is_empty());
        // JPEG magic number: FF D8 FF
        assert_eq!(&data[0..3], &[255, 216, 255]);
    }

    #[test]
    fn to_jpeg_accepts_maximum_quality() {
        let image_data = create_test_image();
        let processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.to_jpeg_internal(100);

        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(!data.is_empty());
        assert_eq!(&data[0..3], &[255, 216, 255]);
    }

    #[test]
    fn to_jpeg_accepts_recommended_quality() {
        let image_data = create_test_image();
        let processor = ImageProcessor::from_bytes(&image_data).unwrap();

        let result = processor.to_jpeg_internal(85);

        assert!(result.is_ok());
    }

    #[test]
    fn width_returns_correct_dimension() {
        let image_data = create_test_image();
        let processor = ImageProcessor::from_bytes(&image_data).unwrap();

        assert_eq!(processor.get_width(), 10);
    }

    #[test]
    fn height_returns_correct_dimension() {
        let image_data = create_test_image();
        let processor = ImageProcessor::from_bytes(&image_data).unwrap();

        assert_eq!(processor.get_height(), 10);
    }

    #[test]
    fn dimensions_update_after_resize() {
        let image_data = create_test_image();
        let mut processor = ImageProcessor::from_bytes(&image_data).unwrap();

        processor.resize_internal(5, 8).unwrap();

        // Original is 10x10, resizing to 5x8 should give us 5x5 (maintains aspect ratio)
        assert_eq!(processor.get_width(), 5);
        assert_eq!(processor.get_height(), 5);
    }
}
