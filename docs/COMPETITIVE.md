# Iris — competitive positioning

## Job

Image facts for agents: dimensions, format, metadata, opt-in OCR boxes, crops, and equal-size diffs.

## Wedge

The default call measures the file and does not run OCR. Text is local Tesseract, and only when you set `include_ocr` or `profile: "quality"`. A missing binary is a gap. Geometry stays citeable either way.

## Local-first

Decode, crop, and diff run locally. OCR uses the `tesseract` binary on `PATH`. No API key. No generative vision model.

## Peer anchors (learn; do not clone)

| Peer | Gap we exploit |
| --- | --- |
| Local image-search indexes | Retrieval of similar pictures, not dimensions, OCR boxes, or a citeable crop |
| Tesseract MCP servers that only dump text | A transcription with no file hash, no trust warnings, and no crop |
| Vision-model tools | A generated description: not deterministic, and not a measured pixel box |

## Non-goals

- A cloud account as the default path
- A multi-product repo whose only purpose is to pool attention
- A generated caption as the evidence

## Install

```bash
npx -y @sylphx/iris
```

`@sylphx/iris` is the install. It speaks MCP over stdio.
