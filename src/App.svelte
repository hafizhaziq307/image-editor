<script>
  import { phase } from "./store.js";
  import EditImage from "./components/section/EditImage.svelte";
  import Header from "./components/section/Header.svelte";
  import UploadImage from "./components/section/UploadImage.svelte";
</script>

<main class="h-full w-full">
  <Header />

  {#if $phase === 1}
    <UploadImage />
  {:else if $phase >= 2}
    <EditImage />
  {/if}
</main>
