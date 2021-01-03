<script>
  import {
    allTracks,
    filterQuery,
    filteredTracks,
    currentFilters,
  } from '../../store';

  import {
    createCustomQuery,
    createEmptyQuery,
    categoryFiltering,
    customFiltering
  } from '../../filtering';

  import debounce from '../../debounce';
  import { QUERY_TYPE } from '../../constants';

  let active = false;
  let previousQuery = '';

  const handleInput = debounce(e => {
    const query = e.target.value;
    filterQuery.set(createCustomQuery(e.target.value));

    if (query == '') {
      active = false;
    }
	}, 125)

  function resetFilterQuery() {
    filterQuery.set(createEmptyQuery());
  }

  filterQuery.subscribe(queryData => {
    if (!queryData || queryData.type == QUERY_TYPE.EMPTY) {
      filteredTracks.set($allTracks);
      active = false;
      return;
    }

    const { type, query } = queryData;
    if (type == QUERY_TYPE.CATEGORY) {
      filteredTracks.set(categoryFiltering($allTracks, query, $currentFilters));
    } else {
      filteredTracks.set(customFiltering($allTracks, $filteredTracks, query, previousQuery));
    }

    previousQuery = query;
    active = true;
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
  {#if active}
    <button on:click={resetFilterQuery}>&#10799;</button>
  {/if}
</div>
