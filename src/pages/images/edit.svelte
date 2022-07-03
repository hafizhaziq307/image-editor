<script>
  import {
    currentTool,
    imagePath,
    imageWidth,
    imageHeight,
    imageCurrentWidth,
    imageCurrentHeight,
    queueEdits,
  } from "../../lib/stores/store.js";
  import { path, tauri, notification } from "@tauri-apps/api";
  import Resize from "../../lib/components/tools/resize.svelte";

  let imageSrc;

  $: if ($imagePath != null) {
    imageSrc = tauri.convertFileSrc($imagePath);
  }

  const editImage = async () => {
    await tauri.invoke("edit_image", {
      imagePath: $imagePath,
      queues: $queueEdits,
      downloadDir: await path.downloadDir(),
    });

    notification.sendNotification("download completed");
  };
</script>

<div class="flex w-full p-2">
  <div class="w-full">
    <div class="grid grid-cols-12 gap-4">
      <!-- image to edit -->
      <section
        class="col-span-10 h-[100vh] w-full overflow-y-auto rounded-md bg-white p-4 shadow-xl">
        <div
          style="width: {($imageCurrentWidth / $imageWidth) *
            100}%; height: {($imageCurrentHeight / $imageHeight) * 100}%;">
          <img src={imageSrc} alt="img" class="h-full w-full" />
        </div>
      </section>

      <!-- tool details -->
      <div class="col-span-2 h-[90vh]">
        <section class="h-[80vh] w-full rounded-md bg-white p-4 shadow-xl">
          {#if $currentTool == "resize"}
            <Resize />
          {:else if $currentTool == "crop"}
            <p>crop</p>
          {/if}
        </section>

        <section
          class="mt-4 flex h-[9vh] w-full justify-end rounded-md bg-white p-4 shadow-xl">
          <button
            on:click={editImage}
            class="inline-flex w-full justify-center rounded-md border border-transparent bg-green-100 px-4 py-2 text-sm font-medium text-green-900 hover:bg-green-200"
            >Finish</button>
        </section>
      </div>
    </div>
  </div>
</div>
