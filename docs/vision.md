# Vision — Iris

Iris is the local-first image evidence tool for agents.

- **Identity:** package `@sylphx/iris`, bin `iris`, MCP `io.github.SylphxAI/iris`, site <https://sylphxai.github.io/iris/>.
- **User:** an agent that must read a screenshot, UI capture, form, diagram, chart or photo without a generative vision model.
- **Job:** return deterministic image facts — dimensions, metadata, OCR boxes, layout blocks, regions and crops — as citeable evidence.
- **Promise:** geometry and locators are authoritative; optional OCR, captions and open-vocab objects are explicit, scored and never override deterministic facts.
- **Defaults:** `fast` returns metadata, geometry and layout; `quality` explicitly enables OCR and local semantic helpers; GPS metadata is redacted unless requested.
- **Boundaries:** Iris owns local image facts and pixel evidence. It does not own image generation, generic vision chat, photo galleries, or video timelines.
