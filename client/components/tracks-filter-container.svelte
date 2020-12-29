<script>
  import Track from './track.svelte';
  import { filterQuery } from '../store';

  let previousQuery;
  let filteredTracks;
  export let tracks;

  function filterTracks(query) {
    if (query == '') {
      filteredTracks = tracks;
      return;
    }

    let tracksToFilter = tracks;
    // Only search through already filtered tracks if we simply extend the previous query
    if (previousQuery && query.substring(0, previousQuery.length) == previousQuery) {
      tracksToFilter = filteredTracks;
    }

    // Save the current query before filtering
    previousQuery = query;

    filteredTracks = tracksToFilter.filter(track => {
      const { name, categories } = track;

      if (name.length >= query.length && name.includes(query)) {
        return true;
      }

      // Check if any of the tracks categories match the query
      for (const category of categories) {
        if (category.length >= query.length && category.includes(query)) {
          return true;
        }
      }

      return false;
    });
  }

  filterQuery.subscribe(filterTracks);
</script>

<style>
  div {
    display: grid;
    flex-wrap: wrap;
    flex-direction: column;
    grid-template-columns: repeat(3, 1fr);
    grid-auto-rows: 1fr;
    column-gap: var(--spacing);
    row-gap: var(--spacing);
    width: 100%;
    padding: 0 var(--spacing);
    box-sizing: border-box;
  }

  @media screen and (max-width: 900px) {
    div {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media screen and (max-width: 550px) {
    div {
      grid-template-columns: 1fr;
    }
  }

  @media screen and (max-width: 400px) {
    div {
      padding: 0 var(--spacing-sm);
      row-gap: var(--spacing-sm);
    }
  }
</style>

<div>
  {#each filteredTracks as track}
    <Track {track} />
  {/each}
</div>
