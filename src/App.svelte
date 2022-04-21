<script>
  import { phase } from "./store.js";
  import EditImage from "./components/sections/EditImage.svelte";
  import Header from "./components/sections/Header.svelte";
  import UploadImage from "./components/sections/UploadImage.svelte";
</script>

<main class="h-full w-full">
  <Header />

  {#if $phase === 1}
    <UploadImage />
  {:else if $phase >= 2}
    <EditImage />
  {/if}
</main>
