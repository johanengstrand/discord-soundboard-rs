<script>
  export let label;
  export let favorite;
</script>

<style>
  p {
    padding: 0.15rem 0.4rem;
    background-color: var(--tag-color);
    color: var(--tag-text-color);
    border-radius: var(--border-radius);
    margin: 0;
    margin-top: 0.3rem;
    font-size: 0.8rem;
    text-transform: uppercase;
    font-weight: bold;
  }

  .favorite {
    background-color: var(--tag-color-favorite);
    color: var(--tag-text-color-favorite);
  }
</style>

<p class:favorite>
  {#if favorite}
    favorite
  {:else}
    {label}
  {/if}
</p>
