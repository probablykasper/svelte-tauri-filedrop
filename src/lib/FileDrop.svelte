<script lang="ts">
  import { event } from '@tauri-apps/api'
  import { onDestroy } from 'svelte'

  /**
   * List of allowed file extensions. Disallowed files are filtered out.
   *
   * If it's null (default), all file extensions are allowed.
   */
  export let extensions: null | string[] = null
  /** Handle a file drop of one or more files */
  export let handleFiles: (files: string[]) => void = () => {
    // noop
  }
  /**
   * Handle a file drop of a single file.
   *
   * Note that `handleFile()` is also called, no matter what.
   *
   * This is not called if any disallowed files were filtered out.
   */
  export let handleOneFile: (file: string) => void = () => {
    // noop
  }

  function getValidPaths(paths: string[]) {
    if (extensions === null) {
      return paths
    }
    let validPaths = []
    for (const path of paths) {
      for (const ext of extensions) {
        if (path.endsWith('.' + ext)) {
          validPaths.push(path)
          break
        }
      }
    }
    return validPaths
  }

  export let files: string[] = []
  export let position: null | {x: number, y: number} = null

  const fileDrag = event.listen<{paths:string[]}>('tauri://drag', (e) => {
    position = null
    files = getValidPaths(e.payload.paths)
  })

  onDestroy(async () => {
    const unlisten = await fileDrag
    unlisten()
  })
  const fileDropHover = event.listen<{position: {x: number, y: number}}>('tauri://drop-over', (e) => {
    position = e.payload.position
  })
  onDestroy(async () => {
    const unlisten = await fileDropHover
    unlisten()
  })

  const fileDrop = event.listen<{paths:string[]}>('tauri://drop', (e) => {
    const payload = e.payload.paths
    files = getValidPaths(payload)
    if (files.length > 0) {
      handleFiles(files)
    }
    if (payload.length === 1 && files.length === 1) {
      handleOneFile(files[0])
    }
    files = []
    position = null
  })
  onDestroy(async () => {
    const unlisten = await fileDrop
    unlisten()
  })

  const fileDropCancelled = event.listen('tauri://drag-cancelled', () => {
    files = []
    position = null
  })
  onDestroy(async () => {
    const unlisten = await fileDropCancelled
    unlisten()
  })
</script>

<slot {files} {position}/>
