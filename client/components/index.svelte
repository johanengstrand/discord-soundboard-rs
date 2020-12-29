<script>
  import Header from './header.svelte';
  import TracksFilterContainer from './tracks-filter-container.svelte';

  import { fetchTracks } from '../lib/api';
</script>

<style>
  :global(:root) {
    --background: #0f1117;
    --background-dark: #090b0f;
    --background-light: #1d222d;
    --text-color: #fff;
    --border-color: #4a678c;
    --accent-color: var(--border-color);
    --accent-text-color: #b6cbff;
    --hover-background: rgba(0, 0, 0, 0.1);
    --tag-color: var(--background);
    --tag-color-favorite: #ea9a00;
    --tag-text-color: var(--accent-text-color);
    --tag-text-color-favorite: var(--background);
    --spacing: 1rem;
    --spacing-xsm: 0.3rem;
    --spacing-sm: 0.5rem;
    --spacing-md: 0.75rem;
    --header-height: calc(1.5rem + 2 * (var(--spacing)));
    --border-radius: 5px;
    --box-shadow: 1px 1px 10px black;
    --transition-time: 0.1s;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: var(--background);
    color: var(--text-color);
    font-family: mono;
    display: flex;
    flex-direction: column;
  }

  :global(h1, h2, h3, h4, p) {
    margin: var(--spacing-sm) 0;
  }

  :global(button) {
    background-color: var(--background-light);
    cursor: pointer;
    color: var(--text-color);
    font-weight: bold;
    padding: var(--spacing-sm) var(--spacing);
    border: none;
    position: relative;
    font-family: inherit;
    transition: background-color var(--transition-time);
  }

  :global(input) {
    background-color: var(--background-dark);
    border: 1px solid var(--background-light);
    color: var(--text-color);
    border-radius: var(--border-radius);
    padding: var(--spacing-sm) var(--spacing);
    font-family: inherit;
    transition: border-color var(--transition-time);
  }

  :global(input:focus, input:hover, input.active) {
    border-color: var(--border-color);
    background-color: var(--background-light);
  }

  main {
    padding-bottom: var(--spacing);
    margin-top: var(--spacing)
  }

  @media screen and (max-width: 400px) {
    main {
      padding-bottom: var(--spacing-sm);
      margin-top: var(--spacing-sm)
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
