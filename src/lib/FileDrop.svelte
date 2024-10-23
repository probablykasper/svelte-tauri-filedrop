<script lang="ts">
	import { webview } from '@tauri-apps/api'
	import { onDestroy } from 'svelte'

	let dropzone: HTMLDivElement

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

	/**
	 * Allow the user to drop files anywhere on the window
	 * @deprecated
	 */
	export let global_hover = false

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

	let files: string[] = []
	let over = false

	const current_webview = webview.getCurrentWebview()
	const drag_drop_off = current_webview.onDragDropEvent(async (e) => {
		if (e.payload.type === 'enter') {
			files = getValidPaths(e.payload.paths)
		} else if (e.payload.type === 'drop') {
			files = getValidPaths(e.payload.paths)
			if (files.length > 0) {
				handleFiles(files)
			}
			if (e.payload.paths.length === 1 && files.length === 1) {
				handleOneFile(files[0])
			}
			files = []
		} else if (e.payload.type === 'leave') {
			files = []
		} else if (e.payload.type === 'over') {
			const hovered_el = document.elementFromPoint(e.payload.position.x, e.payload.position.y)
			over = dropzone.contains(hovered_el) || global_hover
		}
	})
	onDestroy(async () => {
		;(await drag_drop_off)()
	})
</script>

<div bind:this={dropzone}>
	<slot files={over ? files : []} />
</div>
