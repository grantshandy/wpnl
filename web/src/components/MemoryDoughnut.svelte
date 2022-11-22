<script>
	import { Doughnut } from "svelte-chartjs";
	import { defaultDoughnutOptions } from "../tools";
	
  import {
    Chart as ChartJS,
    Tooltip,
    ArcElement,
    CategoryScale,
    LineElement,
    LinearScale,
    PointElement,
  } from "chart.js";
  import { chartColors, formatFileSize } from "../tools";

  ChartJS.register(
    Tooltip,
    ArcElement,
    CategoryScale,
    LineElement,
    LinearScale,
    PointElement
  );
	
  export let sysinfo = null;
	
	
  let memoryDoughnutData = {
    labels: ["Used Memory", "Free Memory"],
    datasets: [
      {
        data: [0, 0],
        backgroundColor: chartColors,
      },
    ],
  };
	
	$: if (sysinfo) {
    memoryDoughnutData.datasets[0].data = [
      sysinfo.memory.usedMemory,
      sysinfo.memory.totalMemory - sysinfo.memory.usedMemory,
    ];
	}
</script>

<main>
	<div class="rounded-md p-3 text-center space-y-2 border-2 border-base01 h-full">
		<h3 class="text-xl font-bold">Memory</h3>
		<Doughnut
			class="mx-2"
			data={memoryDoughnutData}
			options={defaultDoughnutOptions}
		/>
		<p class="font-semibold text-sm">
			{formatFileSize(sysinfo.memory.usedMemory, 2)} ({(
			(sysinfo.memory.usedMemory / sysinfo.memory.totalMemory) *
			100
			).toFixed(2)}%) of {formatFileSize(sysinfo.memory.totalMemory, 2)}
		</p>
	</div>
</main>
