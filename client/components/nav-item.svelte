<script>
  import Button from './button.svelte';

  export let label;
  export let callback;
</script>

<style>
  button {
    color: var(--accent-background);
    background-color: transparent;
    border-radius: 0;
    border: none;
    border-left: 1px solid var(--accent-border-color);
    text-transform: uppercase;
  }

  button:last-of-type {
    border-right: 1px solid var(--accent-border-color);
  }

  button:hover {
    background-color: var(--hover-background);
  }
</style>

<button on:click={callback}>
  {label}
</button>
