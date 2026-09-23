# Evidence contract — Iris

There is no separate evidence tool. Each of the four tools returns the evidence on the tool result.

Every tool result includes:

- `envelope_version: "1"`
- `status`, `tool`, `product` (`iris`), `product_version`
- `route` as `{ engine, path }`
- `warnings` and `gaps` (arrays; either may be empty)

`read_image` puts geometry and metadata on `twin` (source file name, mime, dimensions, metadata, trust warnings). OCR is a sibling `ocr` object, and only when `include_ocr` is true or `profile` is `quality`.

An OCR line is `{text, bbox: {x, y, width, height}, confidence}`. The default language is `eng`.

If Tesseract is missing, `status` stays `ok`. `ocr.available` is false, and `gaps` gains `{code: "OCR_UNAVAILABLE", message}`.

`image_probe` evidence is the camelCase `probe` object. `crop_region` evidence is the camelCase `region_evidence` object (`imageBase64` only when requested). `compare_images` evidence is the snake_case `diff` object. Inside a `read_image` region, crop bytes are `image_base64` and only when `include_region_image` is true.

GPS is always redacted. OCR text never replaces dimensions or hashes.
