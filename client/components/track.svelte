<script>
  import { playTrack } from '../lib/api';
  import { currentlyPlaying } from '../store';

  let active = false;
  export let track;

  function enter() {
    active = true;
  }

  function leave() {
    if ($currentlyPlaying != track.name) {
      active = false;
    }
  }

  function attemptPlayback() {
    if (!track || !track?.name) {
      return;
    }

    playTrack(track.path);
    currentlyPlaying.set(track.name);
  }

  currentlyPlaying.subscribe(name => {
    active = name == track.name;
  });
</script>

<style>
  article {
    background-color: var(--background-light);
    color: var(--text-color);
    padding: var(--spacing-sm) var(--spacing);
    margin: 0;
    border-radius: var(--border-radius);
    border: 1px solid var(--border-color);
    min-width: 33%;
    cursor: pointer;
    position: relative;
    transition: background-color var(--transition-time);
  }

  article:hover, .active {
    background-color: var(--border-color);
  }

  article:hover .content, .active .content {
    max-width: calc(100% - 2.5 * var(--spacing));
  }

  .content {
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

  p {
    font-size: 0.8rem;
    font-weight: normal;
  }

  svg {
    height: 2.5rem;
    position: absolute;
    right: var(--spacing-sm);
  }
</style>

<article on:mouseenter={enter} on:mouseleave={leave} on:click={() => attemptPlayback()} class:active>
  <div class="content">
    <div class="metadata">
      <h4>{track.name}</h4>
      <p>{track.category}</p>
    </div>
    {#if active || $currentlyPlaying == track.name}
      {#if $currentlyPlaying == track.name}
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 9v6m4-6v6m7-3a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
      {:else}
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
      {/if}
    {/if}
  <div>
</article>

