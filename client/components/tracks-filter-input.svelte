<script>
  import { filterQuery } from '../store';

  function resetFilterQuery() {
    filterQuery.set('');
  }
</script>

<style>
  div {
    flex: 2;
    position: relative;
  }

  input {
    width: 100%;
    box-sizing: border-box;
  }

  button {
    position: absolute;
    right: 0;
    top: 0;
    height: 100%;
    background-color: transparent;
    border: none;
    color: var(--accent-background);
    font-size: 1.5rem;
    display: flex;
    align-items: center;
    padding: 0 var(--spacing-sm);
  }
</style>

<div>
  <input type="text" placeholder="Filter tracks" bind:value={$filterQuery}/>
  {#if $filterQuery != ''}
    <button on:click={resetFilterQuery}>&#10799;</button>
  {/if}
</div>
