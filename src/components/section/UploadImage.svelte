<script>
  import { dialog, tauri } from "@tauri-apps/api";
  import { phase, imageSource } from "../../store.js";

  let currentImage;

  const uploadImage = async () => {
    currentImage = await dialog.open();

    $imageSource = tauri.convertFileSrc(currentImage);

    if (currentImage != null) {
      phase.update((item) => (item = 2));
    }
  };
</script>

<div class="flex h-[90vh] justify-center items-center w-full p-2">
  <button
    class="bg-blue-600 text-white px-4 py-2 rounded"
    on:click={uploadImage}>Upload Image</button>
</div>
