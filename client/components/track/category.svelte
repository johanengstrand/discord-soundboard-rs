<script>
  import { filterQuery } from '../../store';
  import { ROOT_CATEGORY, FAVORITES_CATEGORY } from '../../constants';
  import { createEmptyQuery, createCategoryQuery } from '../../filtering';

  export let label;
  export let simple;
  export let fontSize;

  function onClick(e) {
    e.stopPropagation();

    let newQuery = label;
    if (newQuery == '') {
      newQuery = ROOT_CATEGORY;
    }

    if ($filterQuery.query == newQuery) {
      filterQuery.set(createEmptyQuery());
    } else {
      filterQuery.set(createCategoryQuery(newQuery));
    }
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

  button.simple {
    background-color: transparent;
    color: var(--text-color);
    margin-top: 0px;
    padding: var(--spacing-sm) var(--spacing-sm);
  }

  button:hover {
    background-color: var(--tag-text-color);
    color: var(--tag-color);
  }

  button.simple:hover {
    background-color: transparent;
    color: var(--accent-color);
  }

  .favorite {
    background-color: var(--tag-color-favorite);
    color: var(--tag-text-color-favorite);
  }

  .root {
    background-color: var(--tag-color-root);
    color: var(--tag-text-color-root);
  }

  .favorite.simple {
    background-color: transparent;
    color: var(--tag-color-favorite);
  }

  .root.simple {
    background-color: transparent;
    color: var(--tag-color-root);
  }
</style>

<button
  class:favorite={label == FAVORITES_CATEGORY}
  class:root={label == '' || label == ROOT_CATEGORY}
  class:simple
  style="font-size: {fontSize};"
  on:click={onClick}
>
  {#if label == ''}
    {ROOT_CATEGORY}
  {:else}
    {label}
  {/if}
</button>
