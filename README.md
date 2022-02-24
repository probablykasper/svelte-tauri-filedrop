# Svelte Tauri FileDrop

[![License](https://img.shields.io/npm/l/svelte-tauri-filedrop.svg)](LICENSE)
[![NPM Version](https://img.shields.io/npm/v/svelte-tauri-filedrop.svg)](https://npmjs.com/package/svelte-tauri-filedrop)
[![NPM Downloads](https://img.shields.io/npm/dm/svelte-tauri-filedrop.svg)](https://npmjs.com/package/svelte-tauri-filedrop)

Tauri file drop handling component for Svelte.

## Install
```
npm install svelte-tauri-filedrop
```

## Usage
```svelte
<script lang="ts">
  import FileDrop from 'svelte-tauri-filedrop'

  function open(paths: string[]) {
    // ...
  }
</script>

<FileDrop extensions={['json']} handleFiles={open} let:files>
  <div class="dropzone" class:droppable={files.length > 0}>
    <h2>Drop JSON files</h2>
  </div>
</FileDrop>

<style>
  .dropzone {
    margin: 20px;
    padding: 20px;
    background: #eee;
  }
  .droppable {
    background: #d6dff0;
  }
</style>
```

## API

### `extensions` property: `string[] | null`
List of allowed file extensions. Disallowed files are filtered out.
If it's null (default), all file extensions are allowed.

### `handleFiles` property: `(string[]) => {}`
Handle a file drop of one or more files

### `handleOneFile` property: `(string[]) => {}`
Handle a file drop of a single file.
Note that `handleFile()` is also called.
This is not called if any disallowed files were filtered out.

### `files` slot property: `string[]`
An array of the currently droppable files, excluding disallowed files.
You can use this variable through a let binding: `let:files`.

## Dev Instructions

### Get started
1. Install Node.js (v14 works)
2. Install Rust (v1.50 works)
3. Follow the [Tauri setup guide](https://tauri.studio/en/docs/getting-started/intro)
4. Run `npm install`

### Commands
- `npm run dev`: Start in dev mode
- `npm run package`: Build and package the component
- `npm run lint`: Lint
- `npm run format`: Format

### Publish new version
1. Update `CHANGELOG.md`
2. Check for errors
    ```
    npm run check
    ```
3. Bump the version number
    ```
    npm version --no-git-tag <version>
    ```
4. Generate the package
    ```
    npm run package
    ```
5. Publish the package
    ```
    npm publish ./package
    ```
6. Commit with a tag in format "v#.#.#"
7. Create GitHub release with release notes
