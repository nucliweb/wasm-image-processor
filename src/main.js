// Import styles
import './style.css';

// Import the WASM module
import init, { ImageProcessor } from '../pkg/wasm_image_processor.js';

// State management
let processor = null;
let originalImageData = null;
let currentImageData = null;

// DOM elements
const fileInput = document.getElementById('file-input');
const fileInfo = document.getElementById('file-info');
const originalCanvas = document.getElementById('original-canvas');
const processedCanvas = document.getElementById('processed-canvas');
const originalStats = document.getElementById('original-stats');
const processedStats = document.getElementById('processed-stats');
const performanceLog = document.getElementById('performance-log');

// Buttons
const resizeBtn = document.getElementById('resize-btn');
const blurBtn = document.getElementById('blur-btn');
const brightnessBtn = document.getElementById('brightness-btn');
const grayscaleBtn = document.getElementById('grayscale-btn');
const resetBtn = document.getElementById('reset-btn');
const downloadBtn = document.getElementById('download-btn');

/**
 * Initializes the WebAssembly module and sets up event listeners
 */
async function main() {
    try {
        // Initialize WASM module
        await init();
        logPerformance('✅ WebAssembly module loaded successfully');

        // Setup event listeners
        fileInput.addEventListener('change', handleFileUpload);
        resizeBtn.addEventListener('click', () => applyOperation('resize'));
        blurBtn.addEventListener('click', () => applyOperation('blur'));
        brightnessBtn.addEventListener('click', () => applyOperation('brightness'));
        grayscaleBtn.addEventListener('click', () => applyOperation('grayscale'));
        resetBtn.addEventListener('click', resetImage);
        downloadBtn.addEventListener('click', downloadImage);
    } catch (error) {
        logPerformance(`❌ Failed to initialize: ${error.message}`, true);
        console.error(error);
    }
}

/**
 * Handles file upload and displays the original image
 */
async function handleFileUpload(event) {
    const file = event.target.files[0];
    if (!file) return;

    // Validate file type
    if (!file.type.match(/^image\/(png|jpeg|webp)$/)) {
        logPerformance('❌ Unsupported file format. Please use PNG, JPEG, or WebP.', true);
        return;
    }

    try {
        // Read file as ArrayBuffer
        const arrayBuffer = await file.arrayBuffer();
        originalImageData = new Uint8Array(arrayBuffer);

        // Create initial processor
        const startTime = performance.now();
        processor = new ImageProcessor(originalImageData);
        const loadTime = performance.now() - startTime;

        // Display original image
        displayImage(originalCanvas, originalImageData);
        updateStats(originalStats, processor.width(), processor.height());

        // Initialize processed view with original
        currentImageData = originalImageData;
        displayImage(processedCanvas, currentImageData);
        updateStats(processedStats, processor.width(), processor.height());

        // Update UI
        fileInfo.textContent = `📄 ${file.name} (${formatFileSize(file.size)})`;
        enableControls(true);

        logPerformance(`📸 Image loaded: ${processor.width()}×${processor.height()}px in ${loadTime.toFixed(2)}ms`);
    } catch (error) {
        logPerformance(`❌ Failed to load image: ${error.message}`, true);
        console.error(error);
    }
}

/**
 * Applies an image processing operation
 */
async function applyOperation(operation) {
    if (!processor || !currentImageData) return;

    try {
        const startTime = performance.now();

        // Recreate processor from current state
        processor = new ImageProcessor(currentImageData);

        // Apply the requested operation
        switch (operation) {
            case 'resize':
                processor.resize(800, 600);
                break;
            case 'blur':
                processor.blur(2.0);
                break;
            case 'brightness':
                processor.brightness(20);
                break;
            case 'grayscale':
                processor.grayscale();
                break;
        }

        // Export as PNG for display
        currentImageData = processor.to_png();
        const processingTime = performance.now() - startTime;

        // Update processed canvas
        displayImage(processedCanvas, currentImageData);
        updateStats(processedStats, processor.width(), processor.height());

        // Log performance
        const operationNames = {
            'resize': 'Resize to 800×600',
            'blur': 'Gaussian Blur (σ=2.0)',
            'brightness': 'Brightness +20',
            'grayscale': 'Grayscale Conversion'
        };
        logPerformance(`⚡ ${operationNames[operation]} completed in ${processingTime.toFixed(2)}ms`);
    } catch (error) {
        logPerformance(`❌ Operation failed: ${error.message}`, true);
        console.error(error);
    }
}

/**
 * Resets the processed image to the original
 */
function resetImage() {
    if (!originalImageData) return;

    try {
        processor = new ImageProcessor(originalImageData);
        currentImageData = originalImageData;

        displayImage(processedCanvas, currentImageData);
        updateStats(processedStats, processor.width(), processor.height());

        logPerformance('🔄 Image reset to original');
    } catch (error) {
        logPerformance(`❌ Reset failed: ${error.message}`, true);
        console.error(error);
    }
}

/**
 * Downloads the processed image as PNG
 */
function downloadImage() {
    if (!currentImageData) return;

    try {
        // Create blob and download link
        const blob = new Blob([currentImageData], { type: 'image/png' });
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.href = url;
        link.download = `processed-${Date.now()}.png`;
        link.click();

        // Cleanup
        URL.revokeObjectURL(url);

        logPerformance('💾 Image downloaded successfully');
    } catch (error) {
        logPerformance(`❌ Download failed: ${error.message}`, true);
        console.error(error);
    }
}

/**
 * Displays image data on a canvas
 */
function displayImage(canvas, imageData) {
    const blob = new Blob([imageData], { type: 'image/png' });
    const url = URL.createObjectURL(blob);
    const img = new Image();

    img.onload = () => {
        // Set canvas dimensions
        canvas.width = img.width;
        canvas.height = img.height;

        // Draw image
        const ctx = canvas.getContext('2d');
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.drawImage(img, 0, 0);

        // Cleanup
        URL.revokeObjectURL(url);
    };

    img.onerror = () => {
        logPerformance('❌ Failed to display image', true);
        URL.revokeObjectURL(url);
    };

    img.src = url;
}

/**
 * Updates image statistics display
 */
function updateStats(element, width, height) {
    const pixels = width * height;
    const megapixels = (pixels / 1_000_000).toFixed(2);
    element.textContent = `${width} × ${height}px (${megapixels} MP)`;
}

/**
 * Enables or disables control buttons
 */
function enableControls(enabled) {
    const buttons = [resizeBtn, blurBtn, brightnessBtn, grayscaleBtn, resetBtn, downloadBtn];
    buttons.forEach(btn => btn.disabled = !enabled);
}

/**
 * Logs performance information
 */
function logPerformance(message, isError = false) {
    // Remove placeholder if it exists
    const placeholder = performanceLog.querySelector('.placeholder');
    if (placeholder) {
        placeholder.remove();
    }

    const entry = document.createElement('div');
    entry.className = `performance-entry${isError ? ' error' : ''}`;
    entry.innerHTML = `
        <div>${message}</div>
        <div style="font-size: 0.8em; color: #999;">${new Date().toLocaleTimeString()}</div>
    `;

    performanceLog.insertBefore(entry, performanceLog.firstChild);

    // Keep only last 10 entries
    while (performanceLog.children.length > 10) {
        performanceLog.removeChild(performanceLog.lastChild);
    }
}

/**
 * Formats file size in human-readable format
 */
function formatFileSize(bytes) {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

// Initialize the application
main();
