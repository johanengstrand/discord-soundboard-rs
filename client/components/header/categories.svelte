<script>
  import Category from '../track/category';
  import { currentCategories } from '../../store';
</script>

<style>
  section {
    background-color: var(--background-dark);
    width: 100%;
    padding: 0.1rem 0;
    display: inline-flex;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
    border-bottom: 1px solid var(--background-light);
  }

  @media screen and (max-width: 400px) {
    button {
      flex-basis: 33%;
    }
  }
</style>

<section>
  {#each Object.keys($currentCategories) as category, i (category)}
    <Category simple={true} label={category} fontSize='0.9rem'/>
  {/each}
</section>
