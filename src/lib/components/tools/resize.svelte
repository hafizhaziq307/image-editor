<script>
  import { get } from "svelte/store";

  import {
    imageCurrentWidth,
    imageCurrentHeight,
    imageWidth,
    imageHeight,
    queueEdits,
  } from "../../stores/store.js";

  const setDimensions = (w, h) => {
    imageCurrentWidth.set(w);
    imageCurrentHeight.set(h);

    if ($queueEdits == null || $queueEdits == '') {
      queueEdits.update(n => n + "resize" + " " + w + " " + h);
    }else{
      queueEdits.update(n => n + "::resize" + " " + w + " " + h);
    }
  };

  let widthImg = $imageCurrentWidth;
  let heightImg = $imageCurrentHeight;

  console.log("width: ", widthImg);
  console.log("height: ", heightImg);
</script>

<div class="flex h-full flex-col justify-between">
  <div class="space-y-5">
    <div>
      <span>width</span>
      <input
        type="text"
        bind:value={widthImg}
        class="w-full rounded border border-gray-300 px-2 py-1 focus:border-blue-600 focus:outline-none" />
    </div>

    <div>
      <span>height</span>
      <input
        type="text"
        bind:value={heightImg}
        class="w-full rounded border border-gray-300 px-2 py-1 focus:border-blue-600 focus:outline-none" />
    </div>
  </div>

  <div class="text-center">
    <button
      on:click={setDimensions(widthImg, heightImg)}
      class="inline-flex w-full justify-center rounded-md border border-transparent bg-blue-100 px-4 py-2 text-sm font-medium text-blue-900 hover:bg-blue-200 active:ring-2 active:ring-offset-4"
      >Resize</button>
  </div>
</div>
