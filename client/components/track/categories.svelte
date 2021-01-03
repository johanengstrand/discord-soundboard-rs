<script>
    import TrackCategory from "./category";
    import { ROOT_CATEGORY, FAVORITES_CATEGORY } from "../../constants";

    export let categories;
    export let collections;
</script>

<style>
    .categories {
        height: 1.4rem;
        margin-top: 0.3rem;
        display: flex;
        flex-direction: row;
        justify-content: flex-start;
        column-gap: var(--spacing-xsm);
    }
</style>

<section class="categories">
    {#each categories as category, i (category)}
        <TrackCategory label="{category}" />
    {/each}
    {#each collections as collection, i (collection)}
        <TrackCategory label="{collection}" />
    {/each}
</section>
