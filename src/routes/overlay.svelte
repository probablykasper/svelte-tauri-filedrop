<script lang="ts">
  import FileDrop from '$lib/FileDrop.svelte'
  import { fade } from 'svelte/transition'

  let droppedFiles: string[] = []
  function open(paths: string[]) {
    droppedFiles = paths
  }
</script>

<h1>Overlay Example</h1>

<FileDrop handleFiles={open} let:files>
  {#if files.length > 0}
    <div class="overlay" transition:fade={{ duration: 100 }}>
      <h2>Drop now</h2>
    </div>
  {/if}
</FileDrop>
{#each droppedFiles as file}
  <div>{file}</div>
{/each}

<style lang="sass">
  .overlay
    position: absolute
    width: 100%
    height: 100%
    top: 0px
    left: 0px
    display: flex
    align-items: center
    justify-content: center
    background: hsla(0deg, 0%, 0%, 0.1)
    h2
      padding: 20px
      background: #ffffff
      padding: 35px 60px
      border-radius: 10px
</style>
