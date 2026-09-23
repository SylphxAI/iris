import { defineConfig } from 'vitepress';

/**
 * Iris documentation.
 * Assets are files in this package. No remote fonts, scripts, or images.
 */
export default defineConfig({
  base: '/iris/',
  cleanUrls: true,
  title: 'Iris',
  description:
    'Iris — image facts with pixel-level proof. Dimensions, format, and metadata by default; local Tesseract OCR only when requested.',

  appearance: 'dark',
  lastUpdated: true,
  lang: 'en-US',

  srcExclude: ['adr/**', 'specs/**', 'RESEARCH_CARD.md'],

  vite: {
    build: {
      target: 'esnext',
    },
  },

  head: [
    ['meta', { name: 'theme-color', content: '#7eb6ff' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Iris — image facts with pixel-level proof' }],
    [
      'meta',
      {
        property: 'og:description',
        content:
          'Dimensions, format, and metadata by default. Local Tesseract OCR only when requested.',
      },
    ],
    ['meta', { property: 'og:url', content: 'https://sylphxai.github.io/iris/' }],
    ['meta', { property: 'og:site_name', content: 'Iris' }],
    ['meta', { name: 'twitter:card', content: 'summary' }],
    ['meta', { name: 'twitter:title', content: 'Iris — image facts with pixel-level proof' }],
    [
      'meta',
      {
        name: 'twitter:description',
        content:
          'Dimensions, format, and metadata by default. Local Tesseract OCR only when requested.',
      },
    ],
    ['meta', { name: 'author', content: 'Sylphx' }],
    ['meta', { name: 'robots', content: 'index, follow' }],
    ['link', { rel: 'canonical', href: 'https://sylphxai.github.io/iris/' }],
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/iris/logo.svg' }],
  ],

  themeConfig: {
    logo: '/logo.svg',
    siteTitle: 'Iris',

    nav: [
      { text: 'Quickstart', link: '/guide/quickstart' },
      { text: 'Tools', link: '/reference/tools' },
      { text: 'Compare', link: '/COMPETITIVE' },
    ],

    sidebar: [
      {
        text: 'Get started',
        items: [
          { text: 'Quickstart', link: '/guide/quickstart' },
          { text: 'Defaults', link: '/reference/defaults' },
        ],
      },
      {
        text: 'Tools',
        items: [
          { text: 'Tool surface', link: '/TOOL_SURFACE' },
          { text: 'Tool reference', link: '/reference/tools' },
        ],
      },
      {
        text: 'Product',
        items: [
          { text: 'Vision', link: '/vision' },
          { text: 'Capabilities', link: '/capabilities' },
          { text: 'Evidence contract', link: '/EVIDENCE_CONTRACT' },
          { text: 'Compare', link: '/COMPETITIVE' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/SylphxAI/iris' },
      { icon: 'npm', link: 'https://www.npmjs.com/package/@sylphx/iris' },
    ],

    editLink: {
      pattern: 'https://github.com/SylphxAI/iris/edit/main/docs/:path',
      text: 'Edit this page on GitHub',
    },

    footer: {
      message: 'MIT licensed · geometry by default, OCR when requested · these docs load no remote assets',
      copyright: 'Copyright 2026 Sylphx',
    },

    outline: { level: [2, 3] },

    search: {
      provider: 'local',
      options: {
        detailedView: true,
      },
    },
  },
});
