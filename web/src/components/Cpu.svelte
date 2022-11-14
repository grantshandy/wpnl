<script lang="ts">
  import { Bar } from "svelte-chartjs";
  import {
    Chart,
    Title,
    Tooltip,
    Legend,
    BarElement,
    CategoryScale,
    LinearScale,
  } from "chart.js";

  Chart.register(
    Title,
    Tooltip,
    Legend,
    BarElement,
    CategoryScale,
    LinearScale
  );

  export let sysinfo = null;

  let roundOff = (num: number, places: number) => {
    const x = Math.pow(10, places);
    return Math.round(num * x) / x;
  };
  
  let currentData = {
    labels: [],
    datasets: [
      {
        label: "Cpu Usage (%)",
        data: [],
      }
    ]
  }

  $: if (sysinfo.cpu) {
    let avg: number = roundOff(
      sysinfo.cpu.reduce((acc: number, v: any) => acc + v.usage, 0) /
        sysinfo.cpu.length,
      2
    );
    
    currentData.labels = ["Average"].concat(sysinfo.cpu.map((_cpu: any, idx: number) => `#${idx + 1}`));
    currentData.datasets[0].data = [avg].concat(sysinfo.cpu.map((cpu: any) => cpu.usage));
  }
</script>

<main>
  {#if sysinfo.cpu}
    <div
      class="w-full grid grid-cols-1 md:grid-cols-2 grid-rows-2 md:grid-rows-1 gap-2"
    >
      <div class="bg-slate-600 rounded-md shadow-md p-3">
        <Bar data={currentData} options={{scales: {yAxis: {suggestedMin: 0, suggestedMax: 100}}}} />
      </div>
      <div class="bg-slate-600 rounded-md shadow-md p-3" />
    </div>
  {/if}
</main>
