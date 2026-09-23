# Positioning — Iris

**One-liner:** Iris — image facts with pixel-level proof. Dimensions, format, and metadata by default; local Tesseract OCR only when requested.

- **User:** an agent that must read a screenshot, UI capture, form, diagram, chart, or photo and cite measured pixels.
- **Job:** return dimensions, format, metadata, opt-in OCR lines, crops, and equal-size image diffs as facts.
- **Promise:** geometry comes from the file. OCR runs only when `include_ocr` is true or `profile` is `quality`, and a missing Tesseract binary is reported.
- **Identity:** package `@sylphx/iris`, bin `iris`, MCP `io.github.SylphxAI/iris`, site <https://sylphxai.github.io/iris/>.
- **Companion tools:** Citra, Cue, Spine, Locus, and Lookout are independent products. Compose them through their public MCP and SDK contracts.

See [vision.md](./vision.md) and [capabilities.md](./capabilities.md) for the destination and the owned capabilities. [TOOL_SURFACE.md](./TOOL_SURFACE.md) is the tool policy and [EVIDENCE_CONTRACT.md](./EVIDENCE_CONTRACT.md) is the result contract.
