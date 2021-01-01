<script>
  import { createCategoryQuery } from '../../filtering';
  import { ROOT_CATEGORY, FAVORITES_CATEGORY } from '../../constants';
  import { filterQuery, currentCategories } from '../../store';

  function applyCategoryFilter(category) {
    filterQuery.set(createCategoryQuery(category));
  }
</script>

<style>
  section {
    background-color: var(--background-dark);
    width: 100%;
    display: inline-flex;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
  }

  button {
    background-color: transparent;
    transition: color var(--transition-time);
  }

  button:hover {
    color: var(--accent-color);
  }

  .favorite {
    color: var(--tag-color-favorite);
  }

  .root {
    color: var(--tag-color-root);
  }

  @media screen and (max-width: 900px) {
    section {
      justify-content: flex-start;
    }
  }

  @media screen and (max-width: 400px) {
    button {
      flex-basis: 33%;
    }
  }
</style>

<section>
  {#each Object.keys($currentCategories) as category, i (category)}
    <button
      on:click={() => applyCategoryFilter(category)}
      class:favorite={category == FAVORITES_CATEGORY}
      class:root={category == ROOT_CATEGORY}
    >
      {category}
    </button>
  {/each}
</section>
