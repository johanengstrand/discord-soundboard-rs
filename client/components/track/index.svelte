<script>
  import {
    playTrack,
    stopTrack,
    favoriteTrack,
    unfavoriteTrack,
  } from '../../api';
  import { currentTrack } from '../../store';
  import { FAVORITES_CATEGORY } from '../../constants';
  import TrackCategories from './categories';

  let favorite = false;
  let active = false;
  export let track;

  function attemptPlayback() {
    if (!track || !track?.name) {
      return;
    }

    // TODO: Handle response from API call
    if ($currentTrack == track.name) {
      stopTrack();
      currentTrack.set(null);
    } else {
      playTrack(track.path);
      currentTrack.set(track.name);
    }

    active = true;
  }

  function toggleFavorite(e) {
    // Prevent event propagation to 'attemptPlayback'
    e.stopPropagation();

    if (favorite) {
      track.categories = track.categories.filter(category => {
        return category != FAVORITES_CATEGORY;
      });
      unfavoriteTrack(track.path);
    } else {
      track.categories.push(FAVORITES_CATEGORY);
      favoriteTrack(track.path);
    }

    favorite = !favorite;

    // Trigger a rerender
    track = track;
  }

  currentTrack.subscribe(name => {
    active = name == track.name;
  });

  active = $currentTrack == track.name;
  favorite = track.categories.includes(FAVORITES_CATEGORY);

  // This will stop the track name from being substringed on each render
  const trackName = track.name.substr(0, track.name.lastIndexOf('.'));
</script>

<style>
  article {
    background-color: var(--background-light);
    color: var(--text-color);
    padding: var(--spacing-sm) var(--spacing-md);
    margin: 0;
    border: 1px solid transparent;
    border-radius: var(--border-radius);
    min-width: 33%;
    cursor: pointer;
    position: relative;
    box-shadow: 2px 2px 10px #0c0f13;
    transition: background-color var(--transition-time);
  }

  article:hover {
    border-color: var(--border-color);
  }

  .active {
    background-color: var(--border-color);
  }

  .content {
    max-width: calc(100% - 2.5 * var(--spacing));
    display: flex;
    flex-direction: row;
    align-items: center;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  h4, p {
    margin: 0;
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

<article on:click={attemptPlayback} class:active={$currentTrack == track.name}>
  <div class="content">
    <section class="metadata">
      <h4>{trackName}</h4>
      <TrackCategories categories={track.categories} />
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

