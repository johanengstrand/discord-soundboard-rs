<script>
  import { playTrack } from '../lib/api';

  export let path;
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
  }

  h4, p {
    margin: 0;
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
  }

  p {
    font-size: 0.8rem;
    font-weight: normal;
  }
</style>

<article on:click={() => playTrack({ path })}>
  <h4>{path.substr(path.lastIndexOf('/') + 1)}</h4>
  <p>{path}</p>
</article>

