<script lang="ts">
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
	
  let swapDoughnutData = {
    labels: ["Used Swap", "Free Swap"],
    datasets: [
      {
        data: [0, 0],
        backgroundColor: chartColors,
      },
    ],
  };
	
	$: if (sysinfo) {
    swapDoughnutData.datasets[0].data = [
      sysinfo.swap.usedSwap,
      sysinfo.swap.totalSwap - sysinfo.swap.usedSwap,
    ];
	}
</script>


<main>
  <div class="rounded-md p-3 text-center space-y-2 border-2 border-base01 h-full">
    <h3 class="text-xl font-bold">Swap</h3>
    <Doughnut
      class="mx-2 w-full"
      data={swapDoughnutData}
      options={defaultDoughnutOptions}
    />
    <p class="font-semibold text-sm">
      {formatFileSize(sysinfo.swap.usedSwap, 2)} ({(
        (sysinfo.swap.usedSwap / sysinfo.swap.totalSwap) *
        100
      ).toFixed(2)}%) of {formatFileSize(sysinfo.swap.totalSwap, 2)}
    </p>
  </div>
</main>
