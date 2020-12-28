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
    const queryLength = query.length;

    // Only search through already filtered tracks if we simply extend the previous query
    if (previousQuery && query.substring(0, previousQuery.length) == previousQuery) {
      tracksToFilter = filteredTracks;
    }

    // Save the current query before filtering
    previousQuery = query;

    filteredTracks = tracksToFilter.filter(track => {
      const { name, category } = track;

      if (name.length < queryLength || !name.includes(query)) {
        if (category.length < queryLength || !category.includes(query)) {
          return false;
        }
      }

      return true;
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
</style>

<div>
  {#each filteredTracks as track}
    <Track {track} />
  {/each}
</div>
