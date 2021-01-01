<script>
  import TrackCategory from './category';
  import { FAVORITES_CATEGORY } from '../../constants';

  export let categories;
</script>

<style>
  .categories {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    column-gap: var(--spacing-xsm);
  }
</style>

<section class="categories">
  {#each categories as category, i (category)}
    <TrackCategory label={category} />
  {/each}
</section>
