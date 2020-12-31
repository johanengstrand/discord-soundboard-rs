<script>
  import Track from './track';
  import { QUERY_TYPE } from '../constants';
  import { filterQuery, currentCategories } from '../store';
  import { customFiltering, categoryFiltering } from '../filtering';

  let previousQuery;
  let filteredTracks;
  export let tracks;

  function filterTracks(queryData) {
    if (!queryData || queryData.type == QUERY_TYPE.EMPTY) {
      filteredTracks = tracks;
      return;
    }

    const { type, query } = queryData;
    if (type == QUERY_TYPE.CATEGORY) {
      filteredTracks = categoryFiltering(tracks, query, $currentCategories);
    } else {
      filteredTracks = customFiltering(tracks, filteredTracks, query, previousQuery);
    }

    previousQuery = query;
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
  {#each filteredTracks as track, i (track.path)}
    <Track {track} />
  {/each}
</div>
