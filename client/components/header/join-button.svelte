<script>
  import { hasJoined } from '../../store';
  import { joinChannel, leaveChannel } from '../../api';

  async function toggleJoin() {
    try {
      if ($hasJoined) {
        await leaveChannel();
      } else {
        await joinChannel();
      }
    } catch (e) {
      // TODO: show error
      console.error(e);
      return;
    }

    hasJoined.set(!$hasJoined);
  }
</script>

<style>
  button {
    width: 4.5rem;
    padding: 0;
    margin-left: var(--spacing-xsm);
    transition: none;
  }
</style>

<button
  class:success={$hasJoined == false}
  class:failure={$hasJoined == true}
  on:click={toggleJoin}
>
  {#if $hasJoined}
    Leave
  {:else}
    Join
  {/if}
</button>
