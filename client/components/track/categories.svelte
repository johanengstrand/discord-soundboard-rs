<script>
  import TrackCategory from './category';
  import { ROOT_CATEGORY, FAVORITES_CATEGORY } from '../../constants';

  export let categories;
</script>

<style>
  .categories {
    height: 1.4rem;
    margin-top: 0.3rem;
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    column-gap: var(--spacing-xsm);
  }
</style>

<section class="categories">
  {#each categories as category, i (category)}
    {#if category != ''}
      <TrackCategory label={category} />
    {/if}
  {/each}
</section>
