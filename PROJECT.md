# Iris

Iris — image facts with pixel-level proof. Dimensions, format, and metadata by default; local Tesseract OCR only when requested.

## Lifecycle

- Lifecycle: `bootstrap`
- Layer: `tooling`
- SOTA family roadmap: [`docs/roadmap/sota-family-roadmap.md`](docs/roadmap/sota-family-roadmap.md)

## Goals

- Local-first MCP package with evidence-first read output and benchmark-gated releases.
- Preserve provenance so agents can cite a source path, a hash, and a pixel bbox.

## Non-Goals

- Hosted auth, billing, storage, tenancy, or customer data retention.
- Default generative LLM vision/language for reading.
