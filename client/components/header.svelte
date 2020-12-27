<script>
  import Button from './button.svelte';
  import NavItem from './nav-item.svelte';

  function filterFavorites() {
    console.log("favorites");
  }

  function showCategories() {
    console.log("categories");
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
  }

  h3, nav {
    flex: 1;
  }

  input {
    flex: 2;
  }

  nav {
    height: 100%;
    display: inline-flex;
    justify-content: flex-end;
  }
</style>

<header>
  <h3>Discord soundboard</h3>
  <input type="text" placeholder="Filter tracks" />
  <nav>
    <NavItem label='Favorites' callback={filterFavorites} />
    <NavItem label='Categories' callback={showCategories} />
  </nav>
</header>
