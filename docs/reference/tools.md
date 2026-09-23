# Tool reference

`read_image`, `image_probe`, `crop_region`, `compare_images`.

Every result includes `status`, `tool`, `product`, `product_version`, `envelope_version`, `route`, `warnings`, and `gaps`. `route` is `{ engine, path }`. A gap means something you asked for did not run. An error means the call did not do the thing you asked.

OCR runs only from `read_image`, and only when `include_ocr` is true or `profile` is `quality`. Explicit `include_ocr: false` wins.

## read_image

One local image. Dimensions, format, metadata, and trust warnings. No generative vision model.

| Argument | Required | Behavior |
| --- | --- | --- |
| `path` | yes | Local file path |
| `profile` | no | `fast` (default when omitted) skips OCR. `quality` runs it. |
| `include_ocr` | no | When set, this flag wins over `profile`. |
| `ocr_languages` | no | Tesseract languages. Default `["eng"]`. |
| `include_metadata` | no | Default true. Set false to skip EXIF. |
| `region` | no | `{x, y, width, height}` crops inside this read. |
| `include_region_image` | no | Default false. Set true for PNG bytes of `region`. |
| `max_file_bytes` | no | Default 33,554,432. |
| `max_pixels` | no | Default 67,108,864. Applied to `region`. |
| `max_region_dimension` | no | Optional longest-side cap for the crop, in pixels. |

The geometry and metadata are on `twin`: `filename`, `mime`, `dimensions` (`width`, `height`), `has_alpha`, `color_space`, `metadata`, `trust_warnings`, and `region_evidence` when you passed `region`.

Crop bytes inside that twin are `image_base64`, and only when `include_region_image` is true.

OCR, when requested, is an `ocr` object: `available`, `route` (`tesseract_tsv`), `languages`, `line_count`, and `lines`. Each line is `{text, bbox: {x, y, width, height}, confidence}`. If Tesseract is missing, `available` is false, `status` stays `ok`, and `gaps` includes `{code: "OCR_UNAVAILABLE", message}`.

GPS fields in metadata are always redacted. EXIF is read. XMP and IPTC are not.

## image_probe

Format, dimensions, pixel count, and source hash. Does not run OCR, crop, or a vision model.

| Argument | Required | Behavior |
| --- | --- | --- |
| `path` | yes | Local file path |
| `max_file_bytes` | no | Default 33,554,432. |

`probe` is camelCase: `format`, `mime`, `width`, `height`, `pixelCount`, `hasAlpha`, `colorType`, `sourceHash`, `fileSize`, `route`.

## crop_region

One citeable region. Does not run OCR.

| Argument | Required | Behavior |
| --- | --- | --- |
| `path` | yes | Local file path |
| `region` | yes | `{x, y, width, height}` inside the image |
| `include_region_image` | no | Default false. Set true for PNG bytes. |
| `max_file_bytes` | no | Default 33,554,432. |
| `max_pixels` | no | Default 67,108,864. |
| `max_region_dimension` | no | Optional longest-side cap, in pixels. |

`region_evidence` is camelCase: `bbox`, `width`, `height`, `pixelCount`, `regionHash`, `mime`, `route`, `resized`, and `imageBase64` only when `include_region_image` is true. A region outside the image is an error.

## compare_images

Changed pixels between two images. Does not run OCR. Both images must have the same width and height; Iris does not resize them.

| Argument | Required | Behavior |
| --- | --- | --- |
| `before` | yes | Local file path |
| `after` | yes | Local file path |
| `threshold` | no | Default `0`. A pixel counts when any channel differs by more than this. |
| `max_file_bytes` | no | Default 33,554,432, applied to each file. |

`diff` is snake_case: `identical`, `width`, `height`, `changed_pixels`, `changed_ratio`, `changed_bbox`, `before_hash`, `after_hash`, `route`. `changed_bbox` is `{x, y, width, height}` or null when nothing changed.
