<script>
    import { currentNotification } from "../store";
    import { NOTIFICATION_TIMEOUT } from "../constants";

    let currentTimeout = null;

    currentNotification.subscribe((notification) => {
        if (currentTimeout) {
            clearTimeout(currentTimeout);
        }

        if (notification) {
            currentTimeout = setTimeout(
                () => currentNotification.set(null),
                NOTIFICATION_TIMEOUT
            );
        }
    });
</script>

<style>
    aside {
        width: 100%;
        height: auto;
        background-color: transparent;
        position: fixed;
        bottom: var(--spacing);
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-end;
        z-index: 1000;
    }

    p {
        padding: var(--spacing) calc(var(--spacing) * 1.5);
        font-weight: bold;
        box-shadow: 1px 2px 20px rgba(0, 0, 0, 0.4);
    }

    p.error {
        background-color: var(--failure-color);
        color: var(--failure-text-color);
        border-radius: var(--border-radius);
    }
</style>

<aside>
    {#if $currentNotification}
        <p class:error="{$currentNotification.error}">
            {$currentNotification.message}
        </p>
    {/if}
</aside>
