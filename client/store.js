import { writable } from 'svelte/store';
import { createEmptyQuery } from './filtering';

export const filterQuery = writable(createEmptyQuery());
export const currentTrack = writable(null);
export const currentNavItem = writable('');
export const currentCategories = writable({});
export const hasJoined = writable(false);
