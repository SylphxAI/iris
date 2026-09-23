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
| Image read | `read_image` | dimensions, format, metadata, trust warnings; OCR lines only when requested |
| Probe | `image_probe` | format, dimensions, pixel count, source hash. No OCR. |
| Region crop | `crop_region` | pixel bounds and hash; PNG bytes only when `include_region_image` is true |
| Image comparison | `compare_images` | changed pixels and a changed box, for equal dimensions. No OCR. |

## Evidence contract

Every result carries a source path or hash, pixel bounds where a region was measured, a route, warnings, and gaps. See [EVIDENCE_CONTRACT.md](./EVIDENCE_CONTRACT.md).

## Not owned

Generative vision as authority, image generation or editing, photo-gallery search, video timelines, and architecture or code claims.
