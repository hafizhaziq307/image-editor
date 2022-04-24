<script>
  import { imageWidth, imageHeight } from "../../stores/store.js";

  let width = $imageWidth;
  let height = $imageHeight;

  const setDimensions = (w, h) => {
    imageWidth.set(w);
    imageHeight.set(h);
  };
</script>

<div class="flex flex-col justify-between h-full">
  <div class="space-y-5">
    <div>
      <div>width</div>
      <input
        type="text"
        bind:value={width}
        class="w-full border border-gray-300 focus:border-blue-600 focus:outline-none rounded px-2 py-1" />
    </div>

    <div>
      <div>height</div>
      <input
        type="text"
        bind:value={height}
        class="w-full border border-gray-300 focus:border-blue-600 focus:outline-none rounded px-2 py-1" />
    </div>
  </div>

  <div class="text-center">
    <button
      on:click={setDimensions(width, height)}
      class="py-2 px-4 rounded bg-blue-600 hover:bg-blue-500 text-sm text-gray-100"
      >Resize</button>
  </div>
</div>
