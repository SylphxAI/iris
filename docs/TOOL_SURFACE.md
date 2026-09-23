# Tool surface — Iris

| Tool | Role |
| --- | --- |
| `read_image` | Dimensions, format, and metadata. OCR only when `include_ocr` is true or `profile` is `quality`. |
| `image_probe` | Format, dimensions, pixel count, and source hash. No OCR. |
| `crop_region` | One region: bounds and hash. PNG bytes only when `include_region_image` is true. No OCR. |
| `compare_images` | Changed pixels between two equal-size images. No OCR. |

CLI: `iris` · SDK: `@sylphx/iris/sdk` · composition with companion tools is through public MCP and SDK contracts only.

`fast`, or no profile, does not run OCR. Explicit `include_ocr: false` wins over `quality`. A missing Tesseract binary leaves `status` at `ok` and adds `OCR_UNAVAILABLE` to `gaps`.
