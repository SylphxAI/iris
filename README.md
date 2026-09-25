> **Merged into [anymd](https://github.com/SylphxAI/anymd) (2026-09-25).** anymd reads PDFs, Office files, EPUB, web pages, images and video into clean Markdown for AI agents: `npx -y @sylphx/anymd`. This repository is archived.

<div align="center">

<img src="docs/public/logo.svg" alt="Iris" width="108" height="108" />

# Iris

### Image facts with pixel-level proof.

**Dimensions, format, and metadata by default. Local Tesseract OCR only when requested.**

[![npm](https://img.shields.io/npm/v/@sylphx/iris?style=flat-square&labelColor=070b0c&color=7eb6ff)](https://www.npmjs.com/package/@sylphx/iris)
[![license](https://img.shields.io/badge/license-MIT-7eb6ff?style=flat-square&labelColor=070b0c)](LICENSE)

**npm** [`@sylphx/iris`](https://www.npmjs.com/package/@sylphx/iris) · **bin** `iris` · **MCP** `io.github.SylphxAI/iris`

</div>

---

## The problem

A screenshot is not a paragraph. An agent that guesses the text, the size, or the pixels it never measured will cite something it cannot show.

## The difference

| The ask | Iris returns |
| --- | --- |
| “Read this screenshot.” | dimensions, format, metadata, and trust warnings. No OCR unless you ask. |
| “Read the text.” | OCR lines with boxes, only when `include_ocr` is true or `profile` is `quality`. |
| “What changed between these two captures?” | changed pixels and a changed box, when both images are the same size. |
| “Crop this region.” | pixel bounds and a hash. PNG bytes only if `include_region_image` is true. |

Iris does not run a generative vision model. A missing Tesseract binary is a gap, not a made-up transcription.

## Install

```bash
npx -y @sylphx/iris
```

That starts a stdio MCP server. No API key. OCR needs the `tesseract` binary on `PATH`.

| Your client | Setup |
| --- | --- |
| **Any agent / CLI** | `npx -y @sylphx/iris` |
| **Claude Code** | `claude mcp add iris -- npx -y @sylphx/iris` |
| **Claude Desktop / Cursor / VS Code / Codex** | `"command": "npx", "args": ["-y", "@sylphx/iris"]` |

```json
{
  "mcpServers": {
    "iris": { "command": "npx", "args": ["-y", "@sylphx/iris"] }
  }
}
```

## A read, then text only if you ask

```json
{ "path": "/absolute/path/to/screenshot.png" }
```

`fast`, or no `profile`, returns dimensions, format, metadata, and trust warnings. It does not run OCR.

```json
{
  "path": "/absolute/path/to/screenshot.png",
  "include_ocr": true
}
```

`include_ocr: true`, or `profile: "quality"`, runs local Tesseract. Explicit `include_ocr: false` wins over `quality`. If Tesseract is missing, `status` stays `ok` and `gaps` includes `OCR_UNAVAILABLE`.

Reference: [tools](https://sylphxai.github.io/iris/reference/tools) · [defaults](https://sylphxai.github.io/iris/reference/defaults)

## Tools

| Tool | When to call it |
| --- | --- |
| `read_image` | One local image. Geometry and metadata. OCR only when requested. An optional `region` crops inside this read. |
| `image_probe` | Format, dimensions, pixel count, and source hash. No OCR and no crop. |
| `crop_region` | One citeable region: bounds and hash. Set `include_region_image` for PNG bytes. No OCR. |
| `compare_images` | Changed pixels between two images of equal dimensions. No OCR. |

## What it will not pretend

- GPS fields are always redacted. There is no switch that puts them back.
- The default file cap is 32 MiB (33,554,432 bytes). A cropped region may not exceed 67,108,864 pixels.
- `compare_images` requires equal dimensions. It does not resize either image.
- OCR lines are `{text, bbox, confidence}`. The default language is `eng`.
- Oversized or undecodable files fail with an error. They are not returned as a successful read.

## Companion MCP tools

| Product | Job |
| --- | --- |
| [Citra](https://github.com/SylphxAI/citra) | PDF answers with page-level proof |
| [Cue](https://github.com/SylphxAI/cue) | Video timelines and timestamp evidence |
| [Spine](https://github.com/SylphxAI/spine) | Repository architecture and impact |
| [Locus](https://github.com/SylphxAI/locus) | Exact code-chunk retrieval |
| [Lookout](https://github.com/SylphxAI/lookout) | Web research with source excerpts |

Each product is independent. Install only the tools your agent needs.

## Documentation

| | |
| --- | --- |
| Website | [sylphxai.github.io/iris](https://sylphxai.github.io/iris/) |
| Quickstart | [Install and first call](https://sylphxai.github.io/iris/guide/quickstart) |
| Compare | [What Iris does, and what it refuses](https://sylphxai.github.io/iris/COMPETITIVE) |

## Development

```bash
bun install
bun test
bun run check
bun run docs:build
cargo test
```

## License

MIT
