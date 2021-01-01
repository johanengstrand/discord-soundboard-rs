<script>
  import {
    FAVORITES_CATEGORY,
    NAV_ITEM_CATEGORIES
  } from '../../constants';
  import { currentNavItem } from '../../store';

  import NavItem from './nav-item';
  import Categories from './categories';
  import JoinButton from './join-button';
  import FilterInput from './tracks-filter-input';

  let showCategoriesMenu = false;

  function showCategories() {
    showCategoriesMenu = true;
  }

  function hideCategories() {
    showCategoriesMenu = false;
  }
</script>

<style>
  header {
    padding: var(--spacing-md) var(--spacing);
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    column-gap: var(--spacing-sm);
    width: 100%;
    box-sizing: border-box;
    background-color: var(--background);
    color: var(--text-color);
  }

  header.separator {
    border-bottom: 1px solid var(--background-light);
  }

  h3, nav {
    flex: 1;
  }

  nav {
    height: 100%;
    display: inline-flex;
    justify-content: flex-end;
    column-gap: var(--spacing-xsm);
  }

  @media screen and (max-width: 900px) {
    h3, nav {
      flex: 0;
    }

    h3 {
      white-space: nowrap;
      margin-right: var(--spacing-sm);
    }
  }

  @media screen and (max-width: 650px) {
    h3 {
      display: none;
    }
  }

  @media screen and (max-width: 400px) {
    header {
      padding: var(--spacing-sm) var(--spacing-sm);
    }
  }
</style>

<header class:separator={!showCategoriesMenu}>
  <h3>Discord soundboard</h3>
  <FilterInput />
  <nav>
    <NavItem label={NAV_ITEM_CATEGORIES} callbackActive={showCategories} callbackInactive={hideCategories} />
    <JoinButton />
  </nav>
</header>

{#if showCategoriesMenu}
  <Categories />
{/if}
