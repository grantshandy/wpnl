<script lang="ts">
	import { chartColors } from "../tools";
	
	export let sysinfo = null;
</script>

<template>
	<div class="rounded-md p-3 text-center space-y-2 border-2 border-base01">
		<h3 class="text-xl font-bold">CPU Cores</h3>
		<div class="grid grid-cols-1 gap-2 grid-rows-{sysinfo.cpu.length}">
			{#each sysinfo.cpu as cpu, i}
				<div class="inline-flex gap-2">
					<p class="text-sm">{i}</p>
					<div class="h-5 rounded-md bg-base01 grow">
						<div
							class="rounded-bl-md rounded-tl-md mx-right h-full progress-transition"
							style="width: {cpu.usage}%; background-color: {chartColors[
							i % chartColors.length
							]}"
						/>
					</div>
				</div>
			{/each}
		</div>
	</div>
</template>

<style>
  .progress-transition {
    transition-delay: 0s;
    transition-duration: 0.25s;
    transition-property: width;
    transition-timing-function: linear;
  }
</style>
