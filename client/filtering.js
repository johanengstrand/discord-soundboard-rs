import { QUERY_TYPE } from './constants';

export function createEmptyQuery() {
  return {
    type: QUERY_TYPE.EMPTY,
    query: '',
  };
}

export function createCategoryQuery(query) {
  return {
    type: QUERY_TYPE.CATEGORY,
    query,
  };
}

export function createCustomQuery(query) {
  return {
    type: QUERY_TYPE.CUSTOM,
    query,
  };
}

export function categoryFiltering(tracks, category, categories) {
  if (!categories || !categories.hasOwnProperty(category.toLowerCase())) {
    return tracks;
  }

  return categories[category];
}

export function customFiltering(tracks, currentTracks, query, previousQuery) {
  let tracksToFilter = tracks;
  // Only search through already filtered tracks if we simply extend the previous query
  if (previousQuery && query.substring(0, previousQuery.length) == previousQuery) {
    tracksToFilter = currentTracks;
  }

  let filteredTracks = tracksToFilter.filter(track => {
    const { name, categories } = track;

    if (
      name.length >= query.length &&
      name.toLowerCase().includes(query.toLowerCase())
    ) {
      return true;
    }

    // Check if any of the tracks categories match the query
    for (const category of categories) {
      if (
        category.length >= query.length &&
        category.toLowerCase().includes(query.toLowerCase())
      ) {
        return true;
      }
    }

    return false;
  });

  console.log(filteredTracks);

  return filteredTracks;
}
