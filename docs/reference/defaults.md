# Predictable defaults

| Profile | Behavior |
| --- | --- |
| `fast` | Deterministic metadata, geometry, and layout |
| `quality` | Explicitly enables OCR and local semantic helpers |
| Optional models | Generative captions and object detection remain opt-in and non-authoritative |

GPS metadata is redacted unless requested. Unsupported inputs and missing
dependencies are returned as warnings or gaps.
