<script>
  import { dialog, tauri } from "@tauri-apps/api";
  import {
    phase,
    imagePath,
    imageWidth,
    imageHeight,
    imageCurrentWidth,
    imageCurrentHeight,
  } from "../../lib/stores/store.js";

  let currentImage;

  const uploadImage = async () => {
    currentImage = await dialog.open();

    if (currentImage == null) return;

    phase.set(2);

    let promise = await tauri.invoke("get_image_dimensions", {
      path: currentImage,
    });

    imagePath.set(currentImage);

    imageWidth.set(promise[0]);
    imageHeight.set(promise[1]);

    imageCurrentWidth.set(promise[0]);
    imageCurrentHeight.set(promise[1]);
  };
</script>

<div class="flex h-[90vh] w-full items-center justify-center p-2">
  <button
    class="rounded-md bg-black/20 px-6 py-3 text-xl font-medium text-white hover:bg-black/30"
    on:click={uploadImage}>Upload Image</button>
</div>
