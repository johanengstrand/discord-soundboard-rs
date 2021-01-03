<script>
    import { onDestroy } from "svelte";
    import { tweened } from "svelte/motion";
    import { linear } from "svelte/easing";

    export let duration;
    export let finishedCallback;
    let timeout = setTimeout(finishedCallback, duration);

    const size = tweened(0, {
        duration: duration,
        easing: linear,
    });

    onDestroy(() => {
        clearTimeout(timeout);
    });

    size.set(100);
</script>

<style>
    div {
        width: 0%;
        height: 100%;
        background-color: var(--progress-bar-color);
        position: absolute;
        top: 0px;
        left: 0px;
        z-index: 1;
    }
</style>

<div style="width: {$size}%"></div>
