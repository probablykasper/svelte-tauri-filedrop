<script lang="ts">
  import FileDrop from '$lib/FileDrop.svelte'
  let file = ''
  function open(path: string) {
    file = path
  }
</script>

<h1>Example</h1>

<p>Drop a JSON file to test</p>

<FileDrop fileExtensions={['json']} handleOneFile={open}>
  <div class="drop-modal">
    <h1>Drop to open</h1>
  </div>
</FileDrop>

{#if file !== ''}
  Dropped file: {file}
{/if}
