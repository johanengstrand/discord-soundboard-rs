<script>
  import Header from './header.svelte';
  import TracksFilterContainer from './tracks-filter-container.svelte';

  import { fetchTracks } from '../lib/api';
</script>

<style>
  :global(:root) {
    --background: #170f17;
    --background-light: #2d1d2d;
    --text-color: #fff;
    --border-color: #754d75;
    --accent-background: #241724;
    --accent-background-light: #ee9eed;
    --accent-border-color: #bd74bd;
    --accent-hover-background-light: #ffd0fe;
    --hover-background: rgba(0, 0, 0, 0.1);
    --spacing: 1rem;
    --spacing-sm: 0.5rem;
    --header-height: calc(1.2rem + 2 * (var(--spacing)));
    --border-radius: 5px;
    --box-shadow: 1px 1px 10px black;
    --transition-time: 0.1s;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: var(--background);
    color: var(--text-color);
    font-family: sans-serif;
    display: flex;
    flex-direction: column;
  }

  :global(h1, h2, h3, h4, p) {
    margin: var(--spacing-sm) 0;
  }

  :global(button) {
    border: 0px;
    background-color: var(--background-light);
    cursor: pointer;
    color: var(--text-color);
    font-weight: bold;
    padding: var(--spacing-sm) var(--spacing);
    border: 1px solid var(--border-color);
    transition: background-color var(--transition-time);
  }

  :global(input) {
    background-color: var(--accent-hover-background-light);
    border: 1px solid var(--accent-border-color);
    color: var(--accent-background);
    border-radius: var(--border-radius);
    padding: var(--spacing-sm) var(--spacing);
    transition: background-color var(--transition-time);
  }

  :global(input:focus, input:hover) {
    background-color: var(--text-color);
  }

  main {
    padding-bottom: var(--spacing);
    margin-top: calc(var(--header-height) + var(--spacing));
  }

  @media screen and (max-width: 400px) {
    main {
      padding-bottom: var(--spacing-sm);
      margin-top: calc(var(--header-height) + var(--spacing-sm));
    }
  }
</style>

<Header />
<main>
  {#await fetchTracks()}
    <p>Fetching tracks..</p>
  {:then tracks}
    <TracksFilterContainer tracks={tracks} />
  {:catch error}
    <p>{error.message}</p>
  {/await}
</main>
