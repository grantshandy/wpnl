<script lang="ts">
	import axios from "axios";

	interface Info {
		name: string;
		kernel_version: string;
		os_version: string;
		host_name: string;
		neofetch: string;
	}

	let info: Info = null;
	let error: string = null;

	axios("/info", { responseType: "json" })
		.then((response) => {
			info = response.data;
		})
		.catch((e) => {
			error = e;
		});
</script>

<main>
	{#if info}
		<div class="">
			{#if info.host_name}
				<p>Hostname: {info.host_name}</p>
			{/if}
			{#if info.name}
				<p>Operating System: {info.name}</p>
			{/if}
			{#if info.kernel_version}
				<p>Kernel Version: {info.kernel_version}</p>
			{/if}
			{#if info.os_version}
				<p>Operating System Version: {info.os_version}</p>
			{/if}
			{#if info.neofetch}
				<p>Neofetch:</p>
				<pre>{info.neofetch}</pre>
			{/if}
		</div>
	{:else if error}
		<p class="font-bold text-red-500">Error: {error}</p>
	{:else}
		<p>Loading...</p>
	{/if}
</main>
