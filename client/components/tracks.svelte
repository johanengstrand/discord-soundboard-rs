<script>
    import Track from "./track";
    import { QUERY_TYPE } from "../constants";
    import { customFiltering, categoryFiltering } from "../filtering";
    import { currentFilters } from "../store";

    export let tracks;
</script>

<style>
    div {
        display: grid;
        flex-wrap: wrap;
        flex-direction: column;
        grid-template-columns: repeat(4, 1fr);
        grid-auto-rows: 1fr;
        column-gap: var(--spacing);
        row-gap: var(--spacing);
        width: 100%;
        padding: 0 var(--spacing);
        box-sizing: border-box;
    }

    .empty {
        grid-area: 1 / 1 / span 1 / span 4;
        text-align: center;
    }

    @media screen and (max-width: 1200px) {
        div {
            grid-template-columns: repeat(3, 1fr);
        }
    }

    @media screen and (max-width: 900px) {
        div {
            grid-template-columns: repeat(2, 1fr);
        }
    }

    @media screen and (max-width: 550px) {
        div {
            grid-template-columns: 1fr;
        }
    }

    @media screen and (max-width: 400px) {
        div {
            padding: 0 var(--spacing-sm);
            row-gap: var(--spacing-sm);
        }
    }
</style>

<div>
    {#each tracks as track, i (track.path)}
        <Track track="{track}" />
    {:else}
        <p class="empty">No tracks found</p>
    {/each}
</div>
