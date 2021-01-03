import { writable } from "svelte/store";
import { createEmptyQuery } from "./filtering";

export const filterQuery = writable(createEmptyQuery());
export const currentNavItem = writable("");
export const currentFilters = writable({});
export const currentNotification = writable(null);
export const hasJoined = writable(false);
export const filteredTracks = writable([]);
export const allTracks = writable([]);
