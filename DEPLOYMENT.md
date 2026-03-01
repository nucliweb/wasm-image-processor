# Deployment Guide

## Build for Production

```bash
# Build WASM and create optimized bundle
npm run build

# Preview production build locally
npm run preview
```

The production build will be in the `dist/` directory.

## Deployment Options

### Option 1: Vercel (Recommended)

Vercel has excellent support for Vite projects.

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy
vercel
```

Or use the [Vercel GitHub integration](https://vercel.com/docs/git) for automatic deployments.

**vercel.json** (optional):
```json
{
  "buildCommand": "npm run build",
  "outputDirectory": "dist"
}
```

### Option 2: Netlify

```bash
# Install Netlify CLI
npm i -g netlify-cli

# Build and deploy
npm run build
netlify deploy --prod --dir=dist
```

**netlify.toml**:
```toml
[build]
  command = "npm run build"
  publish = "dist"

[[headers]]
  for = "/*.wasm"
  [headers.values]
    Content-Type = "application/wasm"
```

### Option 3: GitHub Pages

Create `.github/workflows/deploy.yml`:

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - name: Install wasm-bindgen-cli
        run: cargo install wasm-bindgen-cli

      - name: Install dependencies
        run: npm ci

      - name: Build
        run: npm run build

      - name: Upload artifact
        uses: actions/upload-pages-artifact@v3
        with:
          path: ./dist

  deploy:
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    needs: build
    steps:
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v4
```

Then enable GitHub Pages in repository settings (Settings > Pages > Source: GitHub Actions).

### Option 4: Cloudflare Pages

```bash
# Install Wrangler CLI
npm i -g wrangler

# Build
npm run build

# Deploy
wrangler pages deploy dist
```

Or connect your repository to [Cloudflare Pages](https://pages.cloudflare.com/) for automatic deployments.

**Build settings:**
- Build command: `npm run build`
- Build output directory: `dist`

### Option 5: Self-hosted (Docker)

Create `Dockerfile`:

```dockerfile
FROM node:20-alpine as builder

WORKDIR /app

# Install Rust
RUN apk add --no-cache curl gcc musl-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add wasm target and install wasm-bindgen
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli

# Copy and build
COPY package*.json ./
RUN npm ci

COPY . .
RUN npm run build

# Production stage
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

**nginx.conf**:
```nginx
server {
    listen 80;
    server_name localhost;
    root /usr/share/nginx/html;
    index index.html;

    # WASM MIME type
    types {
        application/wasm wasm;
    }

    location / {
        try_files $uri $uri/ /index.html;
    }

    # Enable gzip compression
    gzip on;
    gzip_types application/wasm application/javascript text/css;
}
```

Build and run:
```bash
docker build -t wasm-image-processor .
docker run -p 8080:80 wasm-image-processor
```

## Important Notes

### WASM MIME Type

Ensure your server serves `.wasm` files with the correct MIME type:
```
application/wasm
```

Most modern hosting platforms handle this automatically.

### CORS Headers

If loading WASM from a different origin, ensure proper CORS headers:
```
Access-Control-Allow-Origin: *
```

### HTTPS Requirement

WebAssembly requires HTTPS in production (except localhost). All recommended platforms provide HTTPS by default.

## Performance Tips

### 1. Enable Compression

Most platforms enable gzip/brotli automatically. Verify in network tab:
- WASM file should be compressed (~300-400KB instead of 800KB)

### 2. CDN Caching

Configure proper cache headers:
```
Cache-Control: public, max-age=31536000, immutable
```

### 3. Preload WASM Module

Add to `index.html` if needed:
```html
<link rel="modulepreload" href="/pkg/wasm_image_processor.js">
```

### 4. Lazy Loading

For larger applications, consider lazy loading the WASM module:
```javascript
const button = document.getElementById('start');
button.addEventListener('click', async () => {
  const { default: init, ImageProcessor } = await import('../pkg/wasm_image_processor.js');
  await init();
  // Use ImageProcessor
});
```

## Verification

After deployment, verify:

1. **Load Test**: Upload and process an image
2. **Performance**: Check processing times in performance log
3. **Console**: No errors in browser console
4. **Network**: Verify WASM loads correctly (Content-Type: application/wasm)
5. **HTTPS**: Confirm site loads over HTTPS

## Environment-Specific Configuration

For different environments, use `.env` files (Vite supports them):

**.env.production**:
```
VITE_API_URL=https://api.production.com
```

Access in code:
```javascript
const apiUrl = import.meta.env.VITE_API_URL;
```

## Monitoring

Consider adding analytics to track:
- WASM load time
- Processing operation performance
- Error rates
- Browser compatibility

Example with simple tracking:
```javascript
window.addEventListener('load', () => {
  const loadTime = performance.now();
  console.log(`Page loaded in ${loadTime}ms`);
});
```

---

**Recommended for this project**: Vercel or Netlify for simplicity, or GitHub Pages if already using GitHub.
