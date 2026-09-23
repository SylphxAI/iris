# Local-first frontier (Iris)

## Principles

1. **No API key on the default path.** `npx -y @sylphx/iris` starts a local stdio server.
2. **Geometry before text.** `fast`, or no profile, returns dimensions, format, and metadata. It does not run OCR.
3. **OCR is local and opt-in.** `quality` or `include_ocr` runs Tesseract on `PATH` and reports a gap when that binary is missing.
4. **No generative vision model.** Iris does not caption an image and does not detect objects.
5. **Four tools.** `read_image`, `image_probe`, `crop_region`, `compare_images`. The last three never run OCR.
6. **Evidence you can point at.** Path, hash, pixel bounds, route, warnings, and gaps. GPS is always redacted.

## What a call does

```
Agent ──MCP──► iris
                 ├─ read_image: dimensions, format, metadata
                 ├─ image_probe: format, dimensions, pixel count, hash
                 ├─ crop_region: one region (PNG bytes only if requested)
                 ├─ compare_images: equal-size pixel diff
                 └─ OCR: tesseract on PATH, only when read_image is asked
```

The default file cap is 32 MiB (33,554,432 bytes). A cropped region may not exceed 67,108,864 pixels. `compare_images` refuses images whose dimensions differ.

## Install

```bash
npx -y @sylphx/iris
```

A path-only read does not start Tesseract:

```json
{ "path": "/absolute/path/to/screenshot.png" }
```

Text is a separate request:

```json
{ "path": "/absolute/path/to/screenshot.png", "include_ocr": true }
```
