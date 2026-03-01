//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_image_processor::ImageProcessor;
use image::{ImageBuffer, Rgb, DynamicImage, ImageFormat};
use std::io::Cursor;

wasm_bindgen_test_configure!(run_in_browser);

/// Creates a simple 20x20 test image
fn create_test_image() -> Vec<u8> {
    let img = ImageBuffer::from_fn(20, 20, |x, _| {
        if x < 10 {
            Rgb([255u8, 0u8, 0u8]) // Red
        } else {
            Rgb([0u8, 0u8, 255u8]) // Blue
        }
    });
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    DynamicImage::ImageRgb8(img)
        .write_to(&mut cursor, ImageFormat::Png)
        .unwrap();
    buffer
}

#[wasm_bindgen_test]
fn wasm_creates_processor_from_valid_image() {
    let image_data = create_test_image();
    let processor = ImageProcessor::new(&image_data);

    assert!(processor.is_ok());
}

#[wasm_bindgen_test]
fn wasm_fails_with_invalid_image() {
    let invalid_data = vec![1, 2, 3, 4, 5];
    let result = ImageProcessor::new(&invalid_data);

    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn wasm_resize_reduces_dimensions() {
    let image_data = create_test_image();
    let mut processor = ImageProcessor::new(&image_data).unwrap();

    let result = processor.resize(10, 10);

    assert!(result.is_ok());
    assert_eq!(processor.width(), 10);
    assert_eq!(processor.height(), 10);
}

#[wasm_bindgen_test]
fn wasm_blur_validates_sigma() {
    let image_data = create_test_image();
    let mut processor = ImageProcessor::new(&image_data).unwrap();

    let result = processor.blur(0.0);

    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn wasm_blur_applies_effect() {
    let image_data = create_test_image();
    let mut processor = ImageProcessor::new(&image_data).unwrap();

    let result = processor.blur(1.5);

    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn wasm_brightness_validates_range() {
    let image_data = create_test_image();
    let mut processor = ImageProcessor::new(&image_data).unwrap();

    let too_low = processor.brightness(-101);
    let too_high = processor.brightness(101);

    assert!(too_low.is_err());
    assert!(too_high.is_err());
}

#[wasm_bindgen_test]
fn wasm_brightness_applies_adjustment() {
    let image_data = create_test_image();
    let mut processor = ImageProcessor::new(&image_data).unwrap();

    let result = processor.brightness(50);

    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn wasm_grayscale_converts_image() {
    let image_data = create_test_image();
    let mut processor = ImageProcessor::new(&image_data).unwrap();

    let result = processor.grayscale();

    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn wasm_exports_to_png() {
    let image_data = create_test_image();
    let processor = ImageProcessor::new(&image_data).unwrap();

    let png_data = processor.to_png();

    assert!(png_data.is_ok());
    let data = png_data.unwrap();
    assert!(!data.is_empty());
    // Verify PNG magic number
    assert_eq!(data[0], 137);
    assert_eq!(data[1], 80);
    assert_eq!(data[2], 78);
    assert_eq!(data[3], 71);
}

#[wasm_bindgen_test]
fn wasm_exports_to_jpeg() {
    let image_data = create_test_image();
    let processor = ImageProcessor::new(&image_data).unwrap();

    let jpeg_data = processor.to_jpeg(85);

    assert!(jpeg_data.is_ok());
    let data = jpeg_data.unwrap();
    assert!(!data.is_empty());
    // Verify JPEG magic number
    assert_eq!(data[0], 255);
    assert_eq!(data[1], 216);
    assert_eq!(data[2], 255);
}

#[wasm_bindgen_test]
fn wasm_jpeg_validates_quality() {
    let image_data = create_test_image();
    let processor = ImageProcessor::new(&image_data).unwrap();

    let zero_quality = processor.to_jpeg(0);
    let over_quality = processor.to_jpeg(101);

    assert!(zero_quality.is_err());
    assert!(over_quality.is_err());
}

#[wasm_bindgen_test]
fn wasm_dimensions_are_accurate() {
    let image_data = create_test_image();
    let processor = ImageProcessor::new(&image_data).unwrap();

    assert_eq!(processor.width(), 20);
    assert_eq!(processor.height(), 20);
}

#[wasm_bindgen_test]
fn wasm_chain_multiple_operations() {
    let image_data = create_test_image();
    let mut processor = ImageProcessor::new(&image_data).unwrap();

    // Chain: resize -> grayscale -> brightness -> blur
    assert!(processor.resize(15, 15).is_ok());
    assert!(processor.grayscale().is_ok());
    assert!(processor.brightness(20).is_ok());
    assert!(processor.blur(1.0).is_ok());

    // Verify final export works
    let png = processor.to_png();
    assert!(png.is_ok());
}
