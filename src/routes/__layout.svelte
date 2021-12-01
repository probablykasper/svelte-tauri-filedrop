<script lang="ts">
  import { goto } from '$app/navigation'
  import { page } from '$app/stores'

  if ($page.path === '/') {
    goto('/main')
  }
</script>

<a href="/main">Main example</a>
<a href="/overlay">Overlay example</a>

{#if $page.path !== '/'}
  <slot />
{/if}
