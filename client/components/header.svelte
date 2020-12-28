<script>
  import Button from './button.svelte';
  import NavItem from './nav-item.svelte';
  import FilterInput from './tracks-filter-input.svelte';
  import { filterQuery } from '../store';

  function filterFavorites() {
    filterQuery.set('favorites');
  }

  function showCategories() {
    console.log('categories');
  }
</script>

<style>
  header {
    padding: 0 var(--spacing);
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    column-gap: var(--spacing-sm);
    width: 100%;
    height: var(--header-height);
    box-sizing: border-box;
    position: fixed;
    top: 0;
    background-color: var(--accent-background-light);
    color: var(--accent-background);
    z-index: 10;
  }

  h3, nav {
    flex: 1;
  }

  nav {
    height: 100%;
    display: inline-flex;
    justify-content: flex-end;
  }
</style>

<header>
  <h3>Discord soundboard</h3>
  <FilterInput />
  <nav>
    <NavItem label='Favorites' callback={filterFavorites} />
    <NavItem label='Categories' callback={showCategories} />
  </nav>
</header>
