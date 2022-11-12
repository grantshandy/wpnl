<script lang="ts">
  import MemoryDoughnut from "./MemoryDoughnut.svelte";
  import SwapDoughnut from "./SwapDoughnut.svelte";

  function formatFileSize(bytes: number, decimalPoint: number): string {
    if (bytes == 0) return "0 Bytes";
    var k = 1000,
      dm = decimalPoint || 2,
      sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"],
      i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + " " + sizes[i];
  }

  interface Stats {
    total_memory: number;
    free_memory: number;
    available_memory: number;
    used_memory: number;
    total_swap: number;
    used_swap: number;
  }

  let statsSource: EventSource = new EventSource("/resources");
  let sysinfo: Stats = null;
  let error: string = null;

  statsSource.onmessage = (event) => {
    sysinfo = JSON.parse(event.data);
  };
</script>

<main>
  {#if sysinfo}
    <div class="w-full grid grid-cols-3 gap-3">
      <MemoryDoughnut sysinfo={sysinfo} />
      <SwapDoughnut sysinfo={sysinfo} />
    </div>
  {:else if error}
    <p class="text-red-500 font-bold">Error: {error}</p>
  {:else}
    <p>Loading...</p>
  {/if}
</main>
