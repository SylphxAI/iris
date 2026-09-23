# Predictable defaults

`fast` means geometry and metadata: dimensions, format, metadata, and trust warnings. It does not run OCR.

`quality`, or `include_ocr: true`, runs local Tesseract and reports a gap when the binary is missing. Explicit `include_ocr: false` wins over `quality`.

`image_probe`, `crop_region`, and `compare_images` do not run OCR. GPS is always redacted. Iris does not call a generative vision model.

The same rules are tabulated in [Defaults](./reference/defaults.md).
