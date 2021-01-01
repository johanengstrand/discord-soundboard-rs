<script>
  import debounce from '../../debounce';
  import { createEmptyQuery, createCustomQuery } from '../../filtering';
  import { filterQuery, currentNavItem } from '../../store';

  let active = false;

  const handleInput = debounce(e => {
    filterQuery.set(createCustomQuery(e.target.value));
	}, 125)

  function resetFilterQuery() {
    filterQuery.set(createEmptyQuery());
  }

  filterQuery.subscribe(queryData => {
    active = queryData.query != '';
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
  <input type="text" placeholder="Filter tracks" on:input={handleInput} class:active value={$filterQuery.query} />
  {#if $filterQuery.query != ''}
    <button on:click={resetFilterQuery}>&#10799;</button>
  {/if}
</div>
