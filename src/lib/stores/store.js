import { writable } from "svelte/store";

export const phase = writable(1);

export const imagePath = writable("");
export const imageWidth = writable("");
export const imageHeight = writable("");

export const currentTool = writable("");

export const queueEdits = writable("");
