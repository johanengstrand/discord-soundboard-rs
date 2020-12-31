<script>
  import { filterQuery } from '../../store';
  import { createCategoryQuery } from '../../filtering';

  export let label;
  export let favorite;

  function onClick(e) {
    e.stopPropagation();
    filterQuery.set(createCategoryQuery(label));
  }
</script>

<style>
  button {
    padding: 0.15rem 0.4rem;
    background-color: var(--tag-color);
    color: var(--tag-text-color);
    border-radius: var(--border-radius);
    margin: 0;
    margin-top: 0.3rem;
    font-size: 0.8rem;
    text-transform: uppercase;
    font-weight: bold;
    transition: background-color color var(--transition-time);
  }

  button:hover {
    background-color: var(--tag-text-color);
    color: var(--tag-color);
  }

  .favorite {
    background-color: var(--tag-color-favorite);
    color: var(--tag-text-color-favorite);
  }
</style>

<button class:favorite on:click={onClick}>
  {#if favorite}
    favorite
  {:else}
    {label}
  {/if}
</button>
