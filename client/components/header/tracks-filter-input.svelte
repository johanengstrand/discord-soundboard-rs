<script>
  import debounce from '../../debounce';
  import { NAV_ITEM_FAVORITES } from '../../constants';
  import { filterQuery, currentNavItem } from '../../store';

  let active = false;

  const handleInput = debounce(e => {
    filterQuery.set(e.target.value);
	}, 125)

  function resetFilterQuery() {
    filterQuery.set('');

    // Reset the currently selected nav item if the favorites item is chosen
    if ($currentNavItem == NAV_ITEM_FAVORITES) {
      currentNavItem.set('');
    }
  }

  filterQuery.subscribe(query => {
    active = query != "";
  });
</script>

<style>
  div {
    flex: 2;
    position: relative;
  }

  input {
    width: 100%;
    box-sizing: border-box;
  }

  button {
    position: absolute;
    right: 0;
    top: 0;
    height: 100%;
    background-color: transparent;
    border: none;
    color: var(--accent-background);
    font-size: 1.5rem;
    display: flex;
    align-items: center;
    padding: 0 var(--spacing-sm);
  }
</style>

<div>
  <input type="text" placeholder="Filter tracks" on:input={handleInput} class:active value={$filterQuery} />
  {#if $filterQuery != ''}
    <button on:click={resetFilterQuery}>&#10799;</button>
  {/if}
</div>
