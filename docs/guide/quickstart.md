# Quickstart

## Install

```bash
npx -y @sylphx/iris
```

For Claude Code:

```bash
claude mcp add iris -- npx -y @sylphx/iris
```

```json
{
  "mcpServers": {
    "iris": { "command": "npx", "args": ["-y", "@sylphx/iris"] }
  }
}
```

No API key. To read text, install the `tesseract` binary and leave it on `PATH`.

## First call

```json
{ "path": "/absolute/path/to/screenshot.png" }
```

`fast`, or no `profile`, returns dimensions, format, metadata, and trust warnings. It does not run OCR.

## When you want the text

```json
{
  "path": "/absolute/path/to/screenshot.png",
  "profile": "quality"
}
```

`quality` or `include_ocr: true` runs local Tesseract. Explicit `include_ocr: false` wins over `quality`. The default language is `eng`; pass `ocr_languages` to name others.

If Tesseract is missing, the read still succeeds: `status` stays `ok`, and `gaps` includes `OCR_UNAVAILABLE`.

## What does not run

`image_probe`, `crop_region`, and `compare_images` do not run OCR. Iris does not call a generative vision model. GPS fields are always redacted.

Inspect the path, hash, pixel bounds, route, warnings, and gaps before treating a result as fact.
