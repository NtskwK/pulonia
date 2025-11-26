import * as path from 'node:path';
import { defineConfig } from 'rspress/config';

export default defineConfig({
  root: path.join(__dirname, 'docs'),
  title: 'Pulonia',
  lang: 'en',
  locales: [
    {
      lang: 'en',
      label: 'English',
      title: 'Pulonia',
      description: 'A static site generator powered by Vite and Vue.',
    },
    {
      lang: 'zh',
      label: '简体中文',
      title: 'Pulonia',
      description: '一个软件OTA升级工具。',
    }
  ],
  icon: '/rspress-icon.png',
  logo: {
    light: '/rspress-light-logo.png',
    dark: '/rspress-dark-logo.png',
  },
  themeConfig: {
    socialLinks: [
      {
        icon: 'github',
        mode: 'link',
        content: 'https://github.com/web-infra-dev/rspress',
      },
    ],
  },
});
