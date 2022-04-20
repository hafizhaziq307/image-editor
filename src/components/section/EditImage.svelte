<script>
  import { imageSource, currentTool } from "../../store.js";
  import Tools from "../../components/tools/Tools.svelte";
  import Resize from "../../components/tools/Resize.svelte";
</script>

<div class="flex w-full p-2">
  <div class="w-full">
    <div class="grid grid-cols-12 grid-rows-6 gap-2  h-[82vh]">
      <!-- list tools available -->
      <section
        class="col-span-2 row-span-3 border rounded-md p-2 border-gray-400">
        <Tools />
      </section>

      <!-- image to edit -->
      <section
        class="border col-span-10 row-span-6 rounded-md p-2 bg-gray-200 border-gray-400 grid place-content-center">
        <img src={$imageSource} alt="img" />
      </section>

      <!-- tool details -->
      <section
        class="border col-span-2 row-span-3 rounded-md p-2 border-gray-400">
        {#if $currentTool == "resize"}
          <Resize />
        {:else if $currentTool == "crop"}
          <p>crop</p>
        {/if}
      </section>
    </div>

    <!-- finish button -->
    <section
      class="border rounded-md p-2 flex justify-end w-full border-gray-400 mt-2">
      <button
        class="bg-green-600 hover:bg-green-500  text-gray-100 px-4 py-2 rounded row-span-1 text-sm"
        >Finish</button>
    </section>
  </div>
</div>
