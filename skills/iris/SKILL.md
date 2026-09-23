# Iris — image facts with pixel-level proof

```bash
npx -y @sylphx/iris
```

Tools: `read_image`, `image_probe`, `crop_region`, `compare_images`.

`fast`, or no profile, returns dimensions, format, and metadata. It does not run OCR.
`quality` or `include_ocr` runs local Tesseract and reports a gap when the binary is missing.
Explicit `include_ocr: false` wins over `quality`. GPS is always redacted. No generative vision model.
