# Quickstart

## Install

```bash
npx -y @sylphx/iris
```

For Claude Code:

```bash
claude mcp add iris -- npx -y @sylphx/iris
```

Then ask one concrete question and inspect the returned locators, route, warnings,
and gaps before relying on the answer.

## Predictable defaults

The `fast` path is deterministic and bounded. Choose `quality` only when OCR,
or local semantic helpers are worth the extra work. Generative captions and
object detection remain opt-in and never replace deterministic geometry.
