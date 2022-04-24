<script>
  import { currentTool } from "../../stores/store.js";

  const updateCurrentTool = (tool) => {
    currentTool.set(tool);
  };
</script>

<div class="flex flex-col text-sm space-y-2">
  <button
    on:click={() => updateCurrentTool("resize")}
    class="rounded border border-blue-600 px-4 py-2 text-sm {$currentTool ==
    'resize'
      ? 'bg-blue-600 text-gray-100'
      : 'hover:bg-gray-50'}">Resize</button>

  <button
    on:click={() => updateCurrentTool("crop")}
    class="rounded border border-blue-600 px-4 py-2 text-sm {$currentTool ==
    'crop'
      ? 'bg-blue-600 text-gray-100'
      : 'hover:bg-gray-50'}">Crop</button>
  <div />

  <button
    on:click={() => updateCurrentTool("flip")}
    class="rounded border border-blue-600 px-4 py-2 text-sm {$currentTool ==
    'flip'
      ? 'bg-blue-600 text-gray-100'
      : 'hover:bg-gray-50'}">Flip</button>
  <div />

  <button
    on:click={() => updateCurrentTool("rotate")}
    class="rounded border border-blue-600 px-4 py-2 text-sm {$currentTool ==
    'rotate'
      ? 'bg-blue-600 text-gray-100'
      : 'hover:bg-gray-50'}">Rotate</button>
  <div />
</div>
