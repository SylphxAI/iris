# Capabilities — Iris

## Surfaces

| Surface | Identity |
| --- | --- |
| MCP | `io.github.SylphxAI/iris` over stdio, `npx -y @sylphx/iris` |
| CLI | `iris` |
| SDK | `@sylphx/iris/sdk` |

## Owned capabilities

| Capability | Tool | Evidence |
| --- | --- | --- |
| Image read | `read_image` | dimensions, format, hash, metadata, OCR lines/words with boxes, layout, agent map |
| Cheap probe | `image_probe` | metadata and geometry without full OCR |
| Region crop | `crop_region` | pixel region with bbox and provenance |
| Image comparison | `compare_images` | deterministic pixel diff and changed-region bbox |

## Evidence contract

Every result carries source path/hash, pixel locators, extraction route, warnings and gaps. See [EVIDENCE_CONTRACT.md](./EVIDENCE_CONTRACT.md).

## Not owned

Generative vision as authority, image generation or editing, photo-gallery search, video timelines, and architecture or code claims.
