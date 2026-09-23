---
"@sylphx/iris": minor
---

`image_probe` and `crop_region` join the public tool surface, and OCR runs only when requested.

`fast`, or no profile, returns dimensions, format, and metadata. `quality` or `include_ocr` runs local Tesseract and reports a gap when that binary is missing. Explicit `include_ocr: false` wins over `quality`.
