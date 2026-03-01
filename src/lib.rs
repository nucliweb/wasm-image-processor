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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brightness_range_validation() {
        // This test would need a valid image, simplified for demonstration
        // In real tests, you'd load a sample image
    }
}
