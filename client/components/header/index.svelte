<script>
  import {
    FAVORITES_CATEGORY,
    NAV_ITEM_FAVORITES,
    NAV_ITEM_CATEGORIES
  } from '../../constants';
  import { filterQuery, currentNavItem } from '../../store';
  import { createEmptyQuery, createCategoryQuery } from '../../filtering';

  import NavItem from './nav-item';
  import FilterInput from './tracks-filter-input';

  function filterFavorites() {
    filterQuery.set(createCategoryQuery(FAVORITES_CATEGORY));
  }

  function filterReset() {
    filterQuery.set(createEmptyQuery());
  }

  function showCategories() {
    console.log('show categories');
  }

  function hideCategories() {
    console.log('hide categories');
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

<header>
  <h3>Discord soundboard</h3>
  <FilterInput />
  <nav>
    <NavItem label={NAV_ITEM_FAVORITES} callbackActive={filterFavorites} callbackInactive={filterReset} />
    <NavItem label={NAV_ITEM_CATEGORIES} callbackActive={showCategories} callbackInactive={hideCategories} />
  </nav>
</header>
