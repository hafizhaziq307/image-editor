<script>
  import { dialog, tauri } from "@tauri-apps/api";
  import {
    phase,
    imagePath,
    imageWidth,
    imageHeight,
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
  };
</script>

<div class="flex h-[90vh] justify-center items-center w-full p-2">
  <button
    class="bg-blue-600 text-white px-4 py-2 rounded"
    on:click={uploadImage}>Upload Image</button>
</div>
