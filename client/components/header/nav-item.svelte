<script>
  import { currentNavItem } from '../../store';

  export let label;
  export let callbackActive;
  export let callbackInactive;

  function toggleActive() {
    if ($currentNavItem == label) {
      currentNavItem.set('');
      callbackActive && callbackInactive();
    } else {
      // Update the currently selected nav item label
      currentNavItem.set(label);
      callbackInactive && callbackActive();
    }
  }
</script>

<style>
  button {
    color: var(--text-color);
    background-color: transparent;
    text-transform: uppercase;
    padding: var(--spacing-sm);
  }

  button:hover::after, button.active::after {
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

<button on:click={toggleActive} class:active={$currentNavItem == label}>
  {label}
</button>
