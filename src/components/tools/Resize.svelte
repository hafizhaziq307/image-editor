<script>
  import { imageObj } from "../../store.js";

  let width = $imageObj.width;
  let height = $imageObj.height;
  let minWidth;
  let minHeight;

  // Simplified fraction by finding the GCD and dividing by it.
  function reduce(number, denomin) {
    var gcd = function gcd(a, b) {
      return b ? gcd(b, a % b) : a;
    };
    gcd = gcd(number, denomin);
    return [number / gcd, denomin / gcd];
  }

  const resize = () => {
    let test = reduce(484, 272);
    // let test = reduce(width, height);
    console.log(test);
  };
</script>

<div class="flex flex-col justify-between h-full">
  <div class="space-y-5">
    <div>
      <div>width</div>
      <input
        type="text"
        value={width}
        class="w-full border border-gray-300 focus:border-blue-600 focus:outline-none rounded px-2 py-1" />
    </div>

    <div>
      <div>height</div>
      <input
        type="text"
        value={height}
        class="w-full border border-gray-300 focus:border-blue-600 focus:outline-none rounded px-2 py-1" />
    </div>
  </div>

  <div class="text-center">
    <button
      on:click={resize}
      class="py-2 px-4 rounded bg-blue-600 hover:bg-blue-500 text-sm text-gray-100"
      >Resize</button>
  </div>
</div>
