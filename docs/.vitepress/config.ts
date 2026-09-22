import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Iris',
  description: 'Image facts with pixel-level proof',
  head: [
    ['link', { rel: 'canonical', href: 'https://sylphxai.github.io/iris/' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'Iris — Image facts with pixel-level proof' }],
    ['meta', { property: 'og:description', content: 'Read screenshots and images with OCR boxes, layout, crops, and deterministic diffs.' }],
    ['meta', { property: 'og:url', content: 'https://sylphxai.github.io/iris/' }],
    ['meta', { name: 'twitter:card', content: 'summary' }]
  ],
  cleanUrls: true,
  themeConfig: {
    nav: [
      { text: 'Quickstart', link: '/guide/quickstart' },
      { text: 'Tools', link: '/reference/tools' },
      { text: 'GitHub', link: 'https://github.com/SylphxAI/iris' },
      { text: 'npm', link: 'https://www.npmjs.com/package/@sylphx/iris' }
    ],
    sidebar: [
      { text: 'Guide', items: [{ text: 'Quickstart', link: '/guide/quickstart' }, { text: 'Predictable defaults', link: '/reference/defaults' }] },
      { text: 'Reference', items: [{ text: 'Tools', link: '/reference/tools' }] }
    ],
    socialLinks: [{ icon: 'github', link: 'https://github.com/SylphxAI/iris' }],
    footer: { message: 'Iris · local-first agent tooling · MIT' }
  }
})
