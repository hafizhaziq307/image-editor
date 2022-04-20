import { writable } from "svelte/store";

export const phase = writable(1);
export const imageSource = writable("");
export const queueEdit = writable([]);
export const currentTool = writable("");
