import { writable } from 'svelte/store';

export const filterQuery = writable('');
export const currentTrack = writable(null);
export const currentNavItem = writable('');
