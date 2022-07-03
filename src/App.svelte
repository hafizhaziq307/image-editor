<script>
  import {
    phase,
    currentTool,
    imageCurrentWidth,
    imageCurrentHeight,
  } from "./lib/stores/store.js";
  import { fly } from "svelte/transition";
  import EditImage from "./pages/images/edit.svelte";
  import UploadImage from "./pages/images/upload.svelte";
  import Icon from "./lib/components/icons/icon.svelte";

  import {
    Popover,
    PopoverButton,
    PopoverPanel,
    Transition,
  } from "@rgossiaux/svelte-headlessui";
</script>

<main class="h-full w-full">
  <!-- navbar -->
  <header class="flex items-center justify-between py-2 px-4 ">
    <!-- logo -->
    <div class="text-white">logo</div>

    {#if $phase >= 2}
      <nav class="flex space-x-10 text-white">
        <button
          type="button"
          on:click={() => currentTool.set("resize")}
          class="text-md rounded-md px-4 py-2 font-medium uppercase  {$currentTool ==
          'resize'
            ? 'bg-black/30'
            : ''}">resize</button>
        <button
          type="button"
          on:click={() => currentTool.set("crop")}
          class="text-md rounded-md px-4 py-2 font-medium uppercase {$currentTool ==
          'crop'
            ? 'bg-black/30'
            : ''}">crop</button>
        <button
          type="button"
          on:click={() => currentTool.set("flip")}
          class="text-md rounded-md px-4 py-2 font-medium uppercase {$currentTool ==
          'flip'
            ? 'bg-black/30'
            : ''}">flip</button>
        <button
          type="button"
          on:click={() => currentTool.set("rotate")}
          class="text-md rounded-md px-4 py-2 font-medium uppercase {$currentTool ==
          'rotate'
            ? 'bg-black/30'
            : ''}">rotate</button>
      </nav>
    {/if}

    <!-- info -->
    <Popover class="relative">
      <PopoverButton>
        <span><Icon name="info" class="h-7 w-7 text-white" /></span>
      </PopoverButton>

      <Transition
        enter="transition duration-200 ease-out"
        enterFrom="translate-y-1 opacity-0"
        enterTo="translate-y-0 opacity-100"
        leave="transition duration-150 ease-in"
        leaveFrom="translate-y-0 opacity-100"
        leaveTo="translate-y-1 opacity-0">
        <PopoverPanel
          class="absolute right-0 z-10 mt-1 w-52 space-y-2 rounded-md bg-white p-4 shadow-xl">
          <div class="text-center font-bold">image editor</div>
          <div class="bg mx-auto h-1 w-12 rounded-full bg-blue-600" />
          <div class="text-center ">ver 0.1.0</div>
          <div class="text-center text-sm">
            &copy; 2022 <a
              href="https://github.com/hafizhaziq307"
              target="_blank"
              class="font-medium hover:text-blue-600 hover:underline"
              >Hafiz Haziq</a>
          </div>
        </PopoverPanel>
      </Transition>
    </Popover>
  </header>

  {#if $phase === 1}
    <UploadImage />
  {:else if $phase >= 2}
    <EditImage />
  {/if}
</main>
