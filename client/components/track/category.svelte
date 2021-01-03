<script>
    import { filterQuery } from "../../store";
    import { ROOT_CATEGORY, FAVORITES_CATEGORY } from "../../constants";
    import { createEmptyQuery, createCategoryQuery } from "../../filtering";

    export let label;
    export let list;
    export let fontSize;

    function onClick(e) {
        e.stopPropagation();

        if ($filterQuery.query == label) {
            filterQuery.set(createEmptyQuery());
        } else {
            filterQuery.set(createCategoryQuery(label));
        }
    }
</script>

<style>
    button {
        height: 100%;
        padding: 0 0.4rem;
        background-color: var(--tag-color);
        color: var(--tag-text-color);
        border-radius: var(--border-radius);
        margin: 0;
        font-size: 0.8rem;
        text-transform: uppercase;
        font-weight: bold;
        transition: background-color color var(--transition-time);
    }

    button.list {
        height: auto;
        background-color: transparent;
        color: var(--text-color);
        margin-top: 0px;
        padding: var(--spacing-xsm) var(--spacing-sm);
    }

    button.list.active {
        background-color: var(--background-light);
    }

    button:hover {
        background-color: var(--tag-text-color);
        color: var(--tag-color);
    }

    button.list:hover {
        background-color: var(--background-light);
        color: var(--text-color);
    }

    button.favorite {
        background-color: var(--tag-color-favorite);
        color: var(--tag-text-color-favorite);
    }

    button.root {
        background-color: var(--tag-color-root);
        color: var(--tag-text-color-root);
    }

    button.list.favorite {
        background-color: transparent;
        color: var(--tag-color-favorite);
    }

    button.list.root {
        background-color: transparent;
        color: var(--tag-color-root);
    }

    button.list.root {
        background-color: transparent;
        color: var(--tag-color-root);
    }

    button.list.favorite:hover {
        color: var(--tag-text-color-favorite);
        background-color: var(--tag-color-favorite);
    }

    button.list.root:hover {
        color: var(--tag-text-color-root);
        background-color: var(--tag-color-root);
    }

    button.list.favorite.active {
        color: var(--tag-text-color-favorite);
        background-color: var(--tag-color-favorite);
    }

    button.list.root.active {
        color: var(--tag-text-color-root);
        background-color: var(--tag-color-root);
    }
</style>

<button
    class:active="{$filterQuery.query == label}"
    class:favorite="{label == FAVORITES_CATEGORY}"
    class:root="{label == ROOT_CATEGORY}"
    class:list
    style="font-size: {fontSize};"
    on:click="{onClick}">
    {label}
</button>
