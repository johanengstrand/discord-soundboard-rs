<script>
  import Category from '../track/category';
  import { currentFilters } from '../../store';
</script>

<style>
  section {
    background-color: var(--background-dark);
    width: 100%;
    padding: 0.4rem 0;
    display: inline-flex;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
    border-bottom: 1px solid var(--background-light);
    column-gap: var(--spacing-sm);
  }

  @media screen and (max-width: 400px) {
    button {
      flex-basis: 33%;
    }
  }
</style>

<section>
  {#each Object.keys($currentFilters) as category, i (category)}
    <Category list={true} label={category} fontSize='0.9rem'/>
  {/each}
</section>
