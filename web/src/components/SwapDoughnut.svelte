<script lang="ts">
  import { Doughnut } from "svelte-chartjs";

  import {
    Chart as ChartJS,
    Title,
    Tooltip,
    ArcElement,
  } from "chart.js";

  ChartJS.register(Title, Tooltip, ArcElement);
  
  export let sysinfo: Stats = null;
	
  let data = {
    labels: ["Used Swap", "Free Swap"],
    datasets: [
      {
        data: [300, 50, 100],
        backgroundColor: [
          "#F7464A",
          "#46BFBD",
        ],
      },
    ],
  };
  
  $: if (sysinfo) {
    data.datasets[0].data[0] = Math.floor(sysinfo.used_swap);
    data.datasets[0].data[1] = Math.floor(sysinfo.total_swap - sysinfo.used_swap);
  }
</script>

<main>
  <div class="w-full h-full bg-slate-600 rounded-md shadow-md p-5 space-y-2">
    <h3 class="text-2xl font-semibold text-center">Swap Usage</h3>
    <Doughnut {data} options={{ responsive: true }} />
  </div>
</main>
