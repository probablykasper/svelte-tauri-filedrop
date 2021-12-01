<script lang="ts">
  import { fade } from 'svelte/transition'
  import { onMount } from 'svelte'
  import type { event } from '@tauri-apps/api'

  let droppable = false
  export let fileExtensions: string[] = []
  export let handleFiles: (files: string[]) => void = () => {
    // noop
  }
  export let handleOneFile: (file: string) => void = () => {
    // noop
  }

  function extractUnlistener(futureUnlistener: Promise<event.UnlistenFn>) {
    return async () => {
      const unlisten = await futureUnlistener
      unlisten()
    }
  }

  function getValidPaths(paths: string[]) {
    let validPaths = []
    for (const path of paths) {
      for (const ext of fileExtensions) {
        if (path.endsWith('.' + ext)) {
          validPaths.push(path)
          break
        }
      }
    }
    return validPaths
  }
  onMount(() => {
    const unlisten = import('@tauri-apps/api').then(({ event }) => {
      return event.listen('tauri://file-drop-hover', (e) => {
        const validPaths = getValidPaths(e.payload as string[])
        if (validPaths.length > 0) {
          droppable = true
        }
      })
    })
    return extractUnlistener(unlisten)
  })
  onMount(() => {
    const unlisten = import('@tauri-apps/api').then(({ event }) => {
      return event.listen('tauri://file-drop', (e) => {
        const validPaths = getValidPaths(e.payload as string[])
        if (validPaths.length > 0) {
          droppable = false
        }
        handleFiles(validPaths)
        if (validPaths.length === 1) {
          handleOneFile(validPaths[0])
        }
      })
    })
    return extractUnlistener(unlisten)
  })
  onMount(() => {
    const unlisten = import('@tauri-apps/api').then(({ event }) => {
      return event.listen('tauri://file-drop-cancelled', () => {
        droppable = false
      })
    })
    return extractUnlistener(unlisten)
  })
</script>

{#if droppable}
  <div class="drag-overlay" transition:fade={{ duration: 100 }}>
    <slot />
  </div>
  <div class="dropzone" />
{/if}

<style lang="sass">
  .dropzone, .drag-overlay
    position: absolute
    width: 100%
    height: 100%
    top: 0px
    left: 0px
  .drag-overlay
    display: flex
    align-items: center
    justify-content: center
    background-color: rgba(#ffffff, 0.2)
    transition: all 100ms ease-in-out
</style>
