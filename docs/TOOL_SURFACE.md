# Tool surface — Iris

| Tool | Role |
| --- | --- |
| `read_image` | Primary image evidence |
| `image_probe` | Cheap probe without full OCR |
| `crop_region` | Citeable crop evidence |
| `compare_images` | Deterministic pixel diff and changed-region bbox |

CLI: `iris` · SDK: `@sylphx/iris/sdk` · composition with companion tools is through public MCP and SDK contracts only.
