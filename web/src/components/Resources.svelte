<script lang="ts">
  import { parse } from "protobufjs";
  import { proto } from "../proto.ts";

  import MemoryDoughnut from "./MemoryDoughnut.svelte";
  import SwapDoughnut from "./SwapDoughnut.svelte";
  import MemoryChart from "./MemoryChart.svelte";
  import CpuDoughnut from "./CpuDoughnut.svelte";
  import CpuCores from "./CpuCores.svelte";
  import CpuChart from "./CpuChart.svelte";

  let root = parse(proto, { keepCase: false }).root;
  let statsProto = root.lookupType("Stats");

  let resources = new WebSocket(`ws://${window.location.host}/resources`);
  let sysinfo = null;
  let error: string = null;

  resources.binaryType = "arraybuffer";
  resources.onmessage = (event) => {
    let bytes = new Uint8Array(event.data);

    sysinfo = statsProto.decode(bytes);
  };
</script>

<main>
  {#if sysinfo}
    <div class="w-full grid grid-cols-2 md:grid-cols-4 gap-2">
      {#if sysinfo.memory}
      <MemoryDoughnut {sysinfo} />
      {/if}
      {#if sysinfo.swap}
      <SwapDoughnut {sysinfo} />
      {/if}
      {#if sysinfo.memory && sysinfo.swap}
      <div class="col-span-2 h-full">
        <MemoryChart {sysinfo} />
      </div>
      {/if}
      {#if sysinfo.cpu}
      <CpuDoughnut {sysinfo} />
      <CpuCores {sysinfo} />
      <div class="col-span-2 h-full">
        <CpuChart {sysinfo} />
      </div>
      {/if}
    </div>
  {:else if error}
    <p class="text-red-500 font-bold">Error: {error}</p>
  {:else}
    <div class="w-full h-full grid place-items-center">
      <p class="font-semibold text-lg">Loading...</p>
    </div>
  {/if}
</main>
