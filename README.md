# Iris

### Image facts with pixel-level proof

Iris gives agents deterministic facts from screenshots, UI captures, forms,
diagrams, charts, and other images. It returns dimensions, metadata, OCR
regions, layout blocks, crops, and trust warnings without requiring a
generative vision model.

```bash
npx -y @sylphx/iris
```

For Claude Code:

```bash
claude mcp add iris -- npx -y @sylphx/iris
```

## The fastest useful workflow

```json
{
  "path": "/absolute/path/to/screenshot.png",
  "include_ocr": true
}
```

The result includes image dimensions, hash, OCR lines with bounding boxes,
layout blocks, a text map for agents, and explicit warnings or gaps.

## Jobs Iris is built for

| Ask your agent | Iris returns |
| --- | --- |
| “Read this screenshot.” | text regions and layout facts |
| “What changed between these UI captures?” | image diff evidence |
| “Extract this form.” | OCR lines and boxes |
| “Crop the important region.” | citeable pixel evidence |
| “Check the image metadata.” | format, dimensions, EXIF trust warnings |

## Tool surface

| Tool | Purpose |
| --- | --- |
| `read_image` | Primary image facts and optional OCR |
| `image_probe` | Cheap metadata and geometry probe |
| `crop_region` | Extract a citeable pixel region |
| `compare_images` | Report pixel and layout differences between two images |

## Predictable defaults

- `fast` returns deterministic metadata, geometry, and layout.
- `quality` explicitly enables OCR and local semantic helpers.
- Generative captions and object detection are opt-in and never authoritative.
- GPS metadata is redacted unless explicitly requested.
- Oversized or unsupported files fail with a structured error.

## Why agents trust it

Every result keeps source path and hash, pixel locators, extraction route,
confidence or warnings, and known gaps. OCR and optional model output never
replace deterministic geometry.

## Companion MCP tools

| Product | Job |
| --- | --- |
| [Citra](https://github.com/SylphxAI/citra) | PDF answers with page-level proof |
| [Cue](https://github.com/SylphxAI/cue) | Video timelines and timestamp evidence |
| [Spine](https://github.com/SylphxAI/spine) | Repository architecture and impact |
| [Locus](https://github.com/SylphxAI/locus) | Exact code-chunk retrieval |
| [Lookout](https://github.com/SylphxAI/lookout) | Web research with source excerpts |

Each product is independent. Install only the tools your agent needs.

## Development

```bash
bun install
bun run build
bun test
cargo test
bun run benchmark:public-proof
bun run benchmark:release-gate
```

## License

MIT
