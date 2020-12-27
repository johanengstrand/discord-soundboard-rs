import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import autoPreprocess from 'svelte-preprocess';

export default {
  input: 'client/index.js',
  output: {
    file: 'public/bundle.js',
    format: 'iife',
    name: 'ui'
  },
  plugins: [
    svelte({ preprocess: autoPreprocess() }),
    resolve({
      extensions: ['.js', '.svelte'],
      browser: true,
      dedupe: ['svelte'],
    }),
  ]
}
