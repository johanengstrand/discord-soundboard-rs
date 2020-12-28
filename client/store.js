import { writable } from 'svelte/store';

export const filterQuery = writable('');
export const currentlyPlaying = writable(null);
