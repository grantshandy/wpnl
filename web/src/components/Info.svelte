<script lang="ts">
	import axios from "axios";
  import { parse } from "protobufjs";
  import { proto } from "../proto.ts";

	let info = null;
	let error: string = null;
	
	let root = parse(proto, { keepCase: false }).root;
	let infoProto = root.lookupType("Info");

	axios("/info", { responseType: "arraybuffer" })
		.then((response) => {
			info = infoProto.decode(new Uint8Array(response.data));
		})
		.catch((e) => {
			error = e;
		});
</script>

<main>
	{#if info}
	<div class="">
		{#if info.hostName}
		<p>Hostname: {info.hostName}</p>
		{/if}
		{#if info.name}
		<p>Operating System: {info.name}</p>
		{/if}
		{#if info.kernelVersion}
		<p>Kernel Version: {info.kernelVersion}</p>
		{/if}
		{#if info.osVersion}
		<p>Operating System Version: {info.osVersion}</p>
		{/if}
	</div>
	{:else if error}
	<p class="font-bold text-red-500">Error: {error}</p>
	{:else}
  <div class="w-full h-full grid place-items-center">
    <p class="font-semibold text-lg">Loading...</p>
  </div>
	{/if}
</main>
