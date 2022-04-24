<script>
  import { phase } from "./lib/stores/store.js";
  import EditImage from "./pages/images/edit.svelte";
  import Header from "./lib/components/headers/header.svelte";
  import UploadImage from "./pages/images/upload.svelte";
</script>

<main class="h-full w-full">
  <Header />

  {#if $phase === 1}
    <UploadImage />
  {:else if $phase >= 2}
    <EditImage />
  {/if}
</main>
