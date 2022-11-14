<script lang="ts">
  import { parse } from "protobufjs";
  import { proto } from "../proto.ts";

  import Memory from "./Memory.svelte";
  import Cpu from "./Cpu.svelte";

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
    <div class="w-full space-y-2">
      <Memory {sysinfo} />
      <Cpu {sysinfo} />
    </div>
  {:else if error}
    <p class="text-red-500 font-bold">Error: {error}</p>
  {:else}
    <div class="w-full h-full grid place-items-center">
      <p class="font-semibold text-lg">Loading...</p>
    </div>
  {/if}
</main>
