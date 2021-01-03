<script>
    import { currentNotification } from "../store";
    import { NOTIFICATION_TIMEOUT } from "../constants";

    currentNotification.subscribe((notification) => {
        if (notification) {
            setTimeout(
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
        position: absolute;
        bottom: var(--spacing);
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: flex-end;
    }

    p {
        padding: var(--spacing) calc(var(--spacing) * 1.5);
        font-weight: bold;
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
