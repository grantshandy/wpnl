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
    labels: ["Used Memory", "Available Memory", "Free Memory"],
    datasets: [
      {
        data: [0, 0, 0],
        backgroundColor: [
          "#F7464A",
          "#FDB45C",
          "#46BFBD",
        ],
      },
    ],
  };
  
  $: if (sysinfo) {
    data.datasets[0].data[0] = Math.floor(sysinfo.used_memory / 10_000);
    data.datasets[0].data[1] = Math.floor(sysinfo.available_memory / 10_000);
    data.datasets[0].data[2] = Math.floor(sysinfo.free_memory / 10_000);
  }
</script>

<main>
  <div class="w-full h-full bg-slate-600 rounded-md shadow-md p-5 space-y-2">
    <h3 class="text-2xl font-semibold text-center">Memory Usage</h3>
    <Doughnut {data} options={{ responsive: true }} />
  </div>
</main>
