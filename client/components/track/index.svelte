<script>
  import {
    playTrack,
    stopTrack,
    favoriteTrack,
    unfavoriteTrack,
  } from '../../api';
  import { FAVORITES_CATEGORY } from '../../constants';
  import { createNotification } from '../../notifications';
  import { filteredTracks, currentFilters, hasJoined } from '../../store';

  import ProgressBar from './progress-bar';
  import TrackCategories from './categories';

  let hidden = false;
  let favorite = false;
  let active = false;
  let duration = 0;
  export let track;

  async function attemptPlayback() {
    if (!track || !track?.name) {
      return;
    }

    // TODO: Handle response from API call
    if (duration != 0) {
      stopTrack(track.path);
      duration = 0;
    } else {
      try {
        duration = await playTrack(track.path);
      } catch (e) {
        createNotification(e, true);
      }
    }

    active = true;
  }

  async function toggleFavorite(e) {
    // Prevent event propagation to 'attemptPlayback'
    e.stopPropagation();
    const categories = $currentFilters;

    try {
      if (favorite) {
        track.categories = track.categories.filter(category => {
          return category != FAVORITES_CATEGORY;
        });
        categories[FAVORITES_CATEGORY] = categories[FAVORITES_CATEGORY].filter(favoritedTrack => {
          return favoritedTrack != track;
        });
        currentFilters.set(categories);
        await unfavoriteTrack(track);
      } else {
        track.categories.push(FAVORITES_CATEGORY);
        categories[FAVORITES_CATEGORY].push(track);
        currentFilters.set(categories);
        await favoriteTrack(track);
      }
    } catch (e) {
      createNotification(e, true);
    }

    favorite = !favorite;

    // Trigger a rerender
    track = track;
  }

  function onTrackFinished() {
    duration = 0;
  }

  hasJoined.subscribe(connected => {
    if (!connected) {
      duration = 0;
    }
  });

  filteredTracks.subscribe(tracks => {
    hidden = !tracks.includes(track);
  });

  active = duration != 0;
  favorite = track.categories.includes(FAVORITES_CATEGORY);

  // This will stop the track name from being substringed on each render
  const trackName = track.name.substr(0, track.name.lastIndexOf('.'));
</script>

<style>
  article {
    background-color: var(--background-light);
    color: var(--text-color);
    padding: var(--spacing-sm) var(--spacing-md);
    padding-right: 0;
    margin: 0;
    border: 1px solid transparent;
    border-radius: var(--border-radius);
    min-width: 33%;
    cursor: pointer;
    position: relative;
    overflow: hidden;
    box-shadow: 2px 2px 10px #0c0f13;
    transition: background-color var(--transition-time);
  }

  article.hidden {
    display: none;
  }

  article:hover {
    border-color: var(--border-color);
  }

  .content {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    position: relative;
    z-index: 2;
  }

  .metadata {
    width: 100%;
  }

  h4, p {
    margin: 0;
  }

  h4 {
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
    max-width: calc(100% - 2.5 * var(--spacing));
  }

  .favorite-wrapper {
    height: 100%;
    position: absolute;
    right: 0;
    padding: 0 var(--spacing-sm);
    z-index: 10;
    background-color: transparent;
    justify-content: center;
    align-items: center;
    display: flex;
  }

  svg {
    height: 1.8rem;
    color: rgba(255, 255, 255, 0.2);
  }

  article:hover svg {
    color: var(--text-color);
  }

  svg.favorite {
    fill: var(--text-color);
    color: var(--text-color);
  }
</style>

<article on:click={attemptPlayback} class:hidden>
  {#if duration != 0}
    <ProgressBar {duration} finishedCallback={onTrackFinished} />
  {/if}
  <div class="content">
    <section class="metadata">
      <h4>{trackName}</h4>
      <TrackCategories categories={track.categories} collections={track.collections} />
    </section>
    <div class="favorite-wrapper" on:click={toggleFavorite} class:active>
      <svg class="w-6 h-6"
        class:favorite
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
        />
      </svg>
    </div>
  <div>
</article>

