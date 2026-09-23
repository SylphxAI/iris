---
layout: home

hero:
  name: Iris
  text: Image facts with pixel-level proof.
  tagline: Dimensions, format, and metadata by default. Local Tesseract OCR only when you ask for it.
  image:
    src: /logo.svg
    alt: Iris
  actions:
    - theme: brand
      text: Quickstart
      link: /guide/quickstart
    - theme: alt
      text: Tool reference
      link: /reference/tools

features:
  - title: Geometry first
    details: fast, or no profile, returns dimensions, format, metadata, and trust warnings. It does not run OCR.
  - title: Text when named
    details: quality or include_ocr runs local Tesseract. A missing binary is a gap, and the read still succeeds.
  - title: Pixels you can cite
    details: Results keep the source path, hash, pixel bounds, route, warnings, and gaps. GPS is always redacted.
---

<div class="lk-section">
  <span class="lk-eyebrow">The difference</span>
  <h2 class="lk-h2">A screenshot is not a paragraph.<br />Measured pixels are.</h2>
  <p class="lk-lead">Iris — image facts with pixel-level proof. Dimensions, format, and metadata by default; local Tesseract OCR only when requested. There is no generative vision model on this path.</p>
  <div class="lk-compare" style="margin-top:28px">
    <div class="side">
      <h3>What a guess says</h3>
      <p>A caption of the screenshot, with no width, no hash, and no box around the words.</p>
    </div>
    <div class="side good">
      <h3>What Iris returns</h3>
      <p>dimensions and format on every read · OCR lines <span class="lk-cite">{text, bbox, confidence}</span> only when requested · a gap when Tesseract is missing.</p>
    </div>
  </div>
</div>

## One image. OCR stays off until you ask.

```json
{ "path": "/absolute/path/to/screenshot.png" }
```

```json
{
  "path": "/absolute/path/to/screenshot.png",
  "include_ocr": true,
  "ocr_languages": ["eng"]
}
```

<p class="lk-fine">The first call is <code>fast</code>. The second runs local Tesseract. <code>include_ocr: false</code> wins over <code>profile: "quality"</code>. PNG bytes for a crop are returned only when <code>include_region_image</code> is true.</p>

<div class="lk-section">
  <span class="lk-eyebrow">How it works</span>
  <h2 class="lk-h2">Three steps from a file to a citation</h2>
  <div class="lk-steps" style="margin-top:26px">
    <div class="lk-step">
      <div class="n">Step 1</div>
      <h3>Add it to your agent</h3>
      <p>One <code>npx</code> line. A stdio MCP server starts for Claude, Cursor, VS Code, Codex, or any other MCP client. No API key.</p>
    </div>
    <div class="lk-step">
      <div class="n">Step 2</div>
      <h3>Read geometry first</h3>
      <p><code>image_probe</code> returns format, dimensions, and a hash, and it never runs OCR. <code>read_image</code> returns dimensions, format, and metadata until you set <code>include_ocr</code> or <code>profile</code> to <code>quality</code>.</p>
    </div>
    <div class="lk-step">
      <div class="n">Step 3</div>
      <h3>Cite the box</h3>
      <p>Use the path, the hash, and the pixel bounds. If OCR was not requested, or Tesseract is missing, do not invent the text.</p>
    </div>
  </div>
</div>
