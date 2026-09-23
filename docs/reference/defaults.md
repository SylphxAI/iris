# Defaults

`fast` means geometry and metadata. `quality` or `include_ocr` runs local Tesseract and reports a gap when it is missing.

| Choice | What runs |
| --- | --- |
| No `profile`, or `profile: "fast"` | Dimensions, format, metadata, and trust warnings. No OCR. |
| `profile: "quality"` | The same read, plus local Tesseract OCR. |
| `include_ocr: true` | OCR, whichever profile you set. |
| `include_ocr: false` | No OCR, even when `profile` is `quality`. |
| Tesseract missing | `status` stays `ok`. `gaps` gets `OCR_UNAVAILABLE`. |

| Limit | Default |
| --- | --- |
| File size (`max_file_bytes`) | 33,554,432 bytes (32 MiB) |
| Crop pixel budget (`max_pixels`) | 67,108,864 |
| OCR language (`ocr_languages`) | `eng` |
| Crop PNG bytes (`include_region_image`) | Off. Crop bytes are omitted. |
| `compare_images` | Both images must have equal dimensions. `threshold` defaults to `0`. |
| Metadata | On for `read_image` (`include_metadata` defaults to true). |
| GPS | Always redacted. |

`image_probe` and `crop_region` never run OCR. A file over the cap, or a file that cannot be decoded, is an error, not a successful read with a warning.
