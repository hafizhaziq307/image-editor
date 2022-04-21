import { writable } from "svelte/store";

export const phase = writable(1);
export const imageObj = writable({
  path: "",
  width: 0,
  height: 0,
});
export const currentTool = writable("");
