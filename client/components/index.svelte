<script>
    import {
        ROOT_CATEGORY,
        TRACK_CATEGORIES_KEY,
        TRACK_COLLECTIONS_KEY,
    } from "../constants";

    import Header from "./header";
    import Tracks from "./tracks";
    import Notifications from "./notifications";

    import { fetchTracks } from "../api";
    import { allTracks, filteredTracks, currentFilters } from "../store";

    function parseFilters(track, parsedFilters, listKey) {
        if (!track.hasOwnProperty(listKey)) {
            console.error(`${listKey} is not a property of track: ${track}`);
            return;
        }

        const keywords = track[listKey];

        if (listKey == TRACK_CATEGORIES_KEY && keywords.length == 0) {
            parsedFilters[ROOT_CATEGORY].push(track);
        } else {
            for (const keyword of track[listKey]) {
                if (keyword == "") {
                    parsedFilters[ROOT_CATEGORY].push(track);
                } else if (parsedFilters.hasOwnProperty(keyword)) {
                    parsedFilters[keyword].push(track);
                } else {
                    parsedFilters[keyword] = [track];
                }
            }
        }
    }

    async function fetchAndParseTracks() {
        const tracks = await fetchTracks();
        const parsedFilters = {
            [ROOT_CATEGORY]: [],
        };

        for (const track of tracks) {
            parseFilters(track, parsedFilters, TRACK_CATEGORIES_KEY);
            parseFilters(track, parsedFilters, TRACK_COLLECTIONS_KEY);
        }

        console.log(parsedFilters);

        allTracks.set(tracks);
        filteredTracks.set(tracks);
        currentFilters.set(parsedFilters);

        return tracks;
    }
</script>

<style>
    :global(:root) {
        --background: #0f1117;
        --background-dark: #090b0f;
        --background-light: #1d222d;
        --text-color: #fff;
        --border-color: #4a678c;
        --accent-color: var(--border-color);
        --accent-text-color: #b6cbff;
        --success-color: #0fff62;
        --success-text-color: #032810;
        --failure-color: #fd3838;
        --failure-text-color: white;
        --progress-bar-color: var(--accent-color);
        --hover-background: rgba(0, 0, 0, 0.1);
        --tag-color: var(--background);
        --tag-color-root: #00ffdb;
        --tag-color-favorite: #ffb629;
        --tag-text-color: var(--accent-text-color);
        --tag-text-color-root: var(--background);
        --tag-text-color-favorite: var(--background);
        --control-height: 2.5rem;
        --spacing: 1rem;
        --spacing-xsm: 0.3rem;
        --spacing-sm: 0.5rem;
        --spacing-md: 0.75rem;
        --header-height: calc(1.5rem + 2 * (var(--spacing)));
        --border-radius: 5px;
        --font-size: 1rem;
        --font-size-sm: 0.9rem;
        --box-shadow: 1px 1px 10px black;
        --transition-time: 0.1s;
    }

    :global(body) {
        margin: 0;
        padding: 0;
        background-color: var(--background);
        color: var(--text-color);
        font-family: mono;
        font-size: var(--font-size);
        display: flex;
        flex-direction: column;
    }

    :global(h1, h2, h3, h4, p) {
        margin: var(--spacing-sm) 0;
    }

    :global(button) {
        background-color: var(--background-light);
        cursor: pointer;
        color: var(--text-color);
        font-weight: bold;
        font-size: var(--font-size-sm);
        padding: var(--spacing-sm) var(--spacing);
        border: none;
        border-radius: var(--border-radius);
        position: relative;
        font-family: inherit;
        transition: background-color var(--transition-time);
    }

    :global(button.cta) {
        background-color: var(--accent-text-color);
        color: var(--background-dark);
    }

    :global(button.failure) {
        background-color: var(--failure-color);
        color: var(--failure-text-color);
    }

    :global(button.cta:hover) {
        filter: brightness(90%);
    }

    :global(button.failure:hover) {
        filter: brightness(110%);
    }

    :global(input) {
        height: var(--control-height);
        background-color: var(--background-dark);
        border: 1px solid var(--background-light);
        color: var(--text-color);
        border-radius: var(--border-radius);
        padding: var(--spacing-sm) var(--spacing);
        font-family: inherit;
        font-size: var(--font-size);
        transition: border-color var(--transition-time);
    }

    :global(input:focus, input:hover, input.active) {
        border-color: var(--border-color);
        background-color: var(--background-light);
    }

    main {
        padding-bottom: var(--spacing);
        margin-top: var(--spacing);
    }

    .status {
        width: 100%;
        text-align: center;
    }

    @media screen and (max-width: 400px) {
        main {
            padding-bottom: var(--spacing-sm);
            margin-top: var(--spacing-sm);
        }
    }
</style>

<Header />
<main>
    {#await fetchAndParseTracks()}
        <p class="status">Fetching tracks...</p>
    {:then tracks}
        <Tracks tracks="{tracks}" />
    {:catch error}
        <p class="status">{error.message}</p>
    {/await}
</main>
<Notifications />
