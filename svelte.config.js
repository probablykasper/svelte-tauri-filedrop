import preprocess from 'svelte-preprocess'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  extensions: ['.svelte', '.md'],
  preprocess: [preprocess()],

  kit: {
    target: 'body',
    ssr: false,
  },
}

export default config
