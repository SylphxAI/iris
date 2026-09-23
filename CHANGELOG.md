# Changelog

## 0.4.1

### Patch Changes

- 40ddea9: Report the package version from the native binary. The multi-arch build compiled without IRIS_PRODUCT_VERSION, so 0.4.0 answered 0.3.3.

## 0.4.0

### Minor Changes

- eda07a4: `image_probe` and `crop_region` join the public tool surface, and OCR runs only when requested.

  `fast`, or no profile, returns dimensions, format, and metadata. `quality` or `include_ocr` runs local Tesseract and reports a gap when that binary is missing. Explicit `include_ocr: false` wins over `quality`.

## 0.3.1

### Patch Changes

- 8192d19: The MCP `initialize` response advertises the docs site, and the package gains discovery keywords.

  `serverInfo.websiteUrl` pointed at the GitHub repository while every companion tool points at its product site; it now advertises `https://sylphxai.github.io/iris/`. The published `keywords` also grow from five generic terms to the real queries users type — image analysis, computer vision, screenshots, UI testing, image diff, EXIF, layout analysis, crops, pixel evidence.

## 0.3.0

- Add `compare_images` for deterministic UI and image diff evidence.
- Add explicit `fast` and `quality` profiles; expensive OCR and local semantics remain opt-in.
- Refresh Iris positioning, documentation, and canonical repository identity.

## 0.2.0

### Breaking

- Brand-sole `@sylphx/iris` (bin `iris`).
- Family envelope v1; Prism retired.

## 0.1.0

### Minor Changes

- 54fac7f: Ship v0.1.0 `read_image` MCP tool with sharp metadata, exifr EXIF, optional Tesseract OCR, and Agent Media Twin output.

All notable changes are documented here. Releases use [Changesets](https://github.com/changesets/changesets).
