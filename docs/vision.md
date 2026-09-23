# Vision — Iris

Iris is the image-evidence tool for agents. Geometry is the default. Text is opt-in.

- **Identity:** package `@sylphx/iris`, bin `iris`, MCP `io.github.SylphxAI/iris`, site <https://sylphxai.github.io/iris/>.
- **User:** an agent that must read a screenshot, UI capture, form, diagram, chart, or photo and cite what it measured.
- **Job:** return dimensions, format, and metadata by default, and local Tesseract OCR only when requested.
- **Promise:** dimensions, hashes, and pixel bounds come from the file. OCR is labeled, scored, and omitted unless you ask. A missing Tesseract binary is a gap.
- **Defaults:** `fast`, or no profile, returns geometry and metadata. `quality` or `include_ocr` runs local Tesseract. Explicit `include_ocr: false` wins. GPS is always redacted.
- **Boundaries:** Iris owns local image facts. It does not own image generation, generic vision chat, photo galleries, or video timelines.
