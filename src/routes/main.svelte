<script lang="ts">
  import FileDrop from '$lib/FileDrop.svelte'

  let droppedFiles: string[] = []
  function open(paths: string[]) {
    droppedFiles = paths
  }
</script>

<h1>Main Example</h1>

<FileDrop extensions={['json']} handleFiles={open} let:files>
  <div class="dropzone" class:droppable={files.length > 0}>
    <h2>Drop JSON files</h2>
    {#each droppedFiles as file}
      <div>{file}</div>
    {/each}
  </div>
</FileDrop>

<style lang="sass">
  .dropzone
    margin: 20px
    padding: 20px
    background: #eee
  .droppable
    background: #d6dff0
</style>
