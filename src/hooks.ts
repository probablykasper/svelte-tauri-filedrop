import type { Handle } from '@sveltejs/kit'

export const handle: Handle = async function ({ event, resolve }) {
  const response = await resolve(event, {
    ssr: false,
  })
  return response
}
