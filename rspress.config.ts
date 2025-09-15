import * as path from 'node:path';
import { defineConfig } from 'rspress/config';

export default defineConfig({
  root: path.join(__dirname, 'docs'),
  title: 'JUWENZANG rust-study',
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
        content: 'https://github.com/juwenzhang/rust-study',
      },
      {
        icon: 'juejin',
        mode: 'link',
        content: 'https://juejin.cn/user/3877322821505440',
      }
    ],
  },
  base: '/rust-study/',
});
