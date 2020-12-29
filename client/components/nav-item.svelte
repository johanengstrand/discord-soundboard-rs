<script>
  export let label;
  export let callback;
</script>

<style>
  button {
    color: var(--text-color);
    background-color: transparent;
    text-transform: uppercase;
    padding: var(--spacing-sm);
  }

  button:hover::after, button:focus::after {
    content: '';
    position: absolute;
    bottom: -0.3rem;
    left: 0px;
    background-color: var(--accent-color);
    width: calc(100% - 3rem);
    height: 0.3rem;
    margin: 0 1.5rem;
    border-radius: var(--border-radius);
  }
</style>

<button on:click={callback}>
  {label}
</button>
