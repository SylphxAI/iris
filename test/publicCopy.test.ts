import { describe, expect, test } from 'bun:test';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';

const root = join(import.meta.dirname, '..');
const description =
  'Iris — image facts with pixel-level proof. ' +
  'Dimensions, format, and metadata by default; local Tesseract OCR only when requested.';

const copyFiles = [
  'README.md',
  'PROJECT.md',
  'skills/iris/SKILL.md',
  'docs/index.md',
  'docs/guide/quickstart.md',
  'docs/reference/defaults.md',
  'docs/reference/tools.md',
  'docs/capabilities.md',
  'docs/vision.md',
  'docs/PREDICTABLE_DEFAULTS.md',
  'docs/POSITIONING.md',
  'docs/TOOL_SURFACE.md',
  'docs/EVIDENCE_CONTRACT.md',
  'docs/COMPETITIVE.md',
  'docs/LOCAL_FIRST_FRONTIER.md',
  'docs/IPPB.md',
  'docs/PUBLISH.md',
];

const banned = [
  'ADR',
  'RFC',
  'docs/specs',
  'docs/adr',
  'smart default',
  'semantic helper',
  'agent map',
  'agent-map',
  'agent_map',
  'unless requested',
  '10k',
  '10,000',
  'image-reader-core',
  'crates/',
  'fastest',
];

const teal = ['#5eead4', '#115e59', '#0f766e', '#14b8a6', '#99f6e4', '#2dd4bf'];

function read(rel: string): string {
  return readFileSync(join(root, rel), 'utf8');
}

describe('public copy', () => {
  test('package description is the geometry-first sentence', () => {
    const parsed = JSON.parse(read('package.json')) as { description: string; keywords?: string[] };
    expect(parsed.description).toBe(description);
    expect(parsed.description.toLowerCase()).not.toContain('layout');
    const keywords = (JSON.parse(read('package.json')) as { keywords: string[] }).keywords;
    expect(keywords).not.toContain('layout-analysis');
  });

  test('registry description stays within the 100-character marketplace limit', () => {
    const parsed = JSON.parse(read('server.json')) as { description: string };
    expect(parsed.description).toBe(
      'Iris — image facts with pixel-level proof. OCR runs only when requested.',
    );
    expect(parsed.description.length).toBeLessThanOrEqual(100);
    expect(parsed.description.toLowerCase()).not.toContain('layout');
  });

  test('public pages do not claim the old default', () => {
    for (const file of copyFiles) {
      const text = read(file);
      const folded = text.toLowerCase();
      for (const phrase of banned) {
        expect(folded.includes(phrase.toLowerCase()), `${file} contains ${phrase}`).toBe(false);
      }
      const layoutLines = text
        .split('\n')
        .filter((line) => line.toLowerCase().includes('layout') && line.trim() !== 'layout: home');
      expect(layoutLines, file).toEqual([]);
    }
  });

  test('the docs theme is blue, not teal', () => {
    const css = read('docs/.vitepress/theme/custom.css');
    const config = read('docs/.vitepress/config.ts');
    expect(css).toContain('#7eb6ff');
    expect(config).toContain('#7eb6ff');
    expect(config).toContain("base: '/iris/'");
    expect(config).toContain("appearance: 'dark'");
    expect(config).toContain("href: '/iris/logo.svg'");
    for (const color of teal) {
      expect(css.toLowerCase()).not.toContain(color);
      expect(config.toLowerCase()).not.toContain(color);
    }
  });
});
