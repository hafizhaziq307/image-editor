<script>
  import { dialog, tauri } from "@tauri-apps/api";
  import { phase, imageObj } from "../../store.js";

  let currentImage;

  const uploadImage = async () => {
    currentImage = await dialog.open();

    if (currentImage != null) $phase = 2;

    let promise = await tauri.invoke("get_image_dimensions", {
      path: currentImage,
    });

    $imageObj.path = currentImage;
    $imageObj.width = promise[0];
    $imageObj.height = promise[1];
  };
</script>

<div class="flex h-[90vh] justify-center items-center w-full p-2">
  <button
    class="bg-blue-600 text-white px-4 py-2 rounded"
    on:click={uploadImage}>Upload Image</button>
</div>
