<script context="module">
    import { hasJoined } from "../../store";
    import { createNotification } from "../../notifications";
    import { isConnected, joinChannel, leaveChannel } from "../../api";

    async function setIsConnected() {
        try {
            let connected = await isConnected();

            if (connected) {
                hasJoined.set(true);
            }
        } catch (e) {
            createNotification(e, true);
        }
    }

    setIsConnected();
</script>

<script>
    let loading = false;

    async function toggleJoin() {
        // Only set loading after a small delay to prevent flickering
        // for fast requests/errors
        const timeout = setTimeout(() => {
            loading = true;
        }, 200);

        try {
            if ($hasJoined) {
                await leaveChannel();
            } else {
                await joinChannel();
            }

            hasJoined.set(!$hasJoined);
        } catch (e) {
            createNotification(e, true);
        }

        clearTimeout(timeout);
        loading = false;
    }
</script>

<style>
    button {
        width: 4.5rem;
        height: var(--control-height);
        margin-left: var(--spacing-xsm);
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        transition: none;
    }

    @keyframes loader {
        0% {
            transform: rotate(0deg);
        }
        50% {
            transform: rotate(180deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    svg {
        width: 1.5rem;
        animation-name: loader;
        animation-duration: 1s;
        animation-iteration-count: infinite;
    }
</style>

<button class:loading class:cta="{$hasJoined == false}" on:click="{toggleJoin}">
    {#if loading}
        <svg
            class="w-6 h-6"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg">
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                ></path>
        </svg>
    {:else if $hasJoined}
        Leave
    {:else}
        Join
    {/if}
</button>
