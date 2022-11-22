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
	<div class="w-full">
		{#if info.hostName}
		<h2 class="text-base1 text-center text-3xl font-bold mt-2">Hostname: {info.hostName}</h2>
		{/if}
		{#if info.name}
		<p class="text-center text-lg font-semibold italic">Operating System: {info.name}
		{#if info.osVersion}
		<span> {info.osVersion}</span>
		{/if}
		</p>
		{/if}
		{#if info.kernelVersion}
		<p class="text-center mx-auto italic">Kernel: {info.kernelVersion}</p>
		{/if}
		<div class="mx-auto w-2/3 text-center mt-2">
			<h3 class="text-xl font-semibold">Load Averages</h3>
			<p>One Minute: {info.loadAvg.one}%</p>
			<p>Five Minutes: {info.loadAvg.five}%</p>
			<p>Fifteen Minutes: {info.loadAvg.fifteen}%</p>
		</div>
	</div>
	{:else if error}
	<p class="font-bold text-red-500">Error: {error}</p>
	{:else}
  <div class="w-full h-full grid place-items-center">
    <p class="font-semibold text-lg">Loading...</p>
  </div>
	{/if}
</main>
