<script lang="ts">
  import { Doughnut } from "svelte-chartjs";
  import { defaultDoughnutOptions, chartColors } from "../tools";
  import {
    Chart as ChartJS,
    Tooltip,
    ArcElement,
    CategoryScale,
    LineElement,
    LinearScale,
    PointElement,
  } from "chart.js";

  ChartJS.register(
    Tooltip,
    ArcElement,
    CategoryScale,
    LineElement,
    LinearScale,
    PointElement
  );

  export let sysinfo = null;

  let roundOff = (num: number, places: number) => {
    const x = Math.pow(10, places);
    return Math.round(num * x) / x;
  };

  let avgCpu: number = 50;
  let avgCpuDoughnutData: any = {
    labels: ["Used CPU", "%"],
    datasets: [
      {
        data: [50, 50],
        backgroundColor: chartColors,
      },
    ],
  };

  $: if (sysinfo) {
    let avg = roundOff(
      sysinfo.cpu.reduce((acc: number, v: any) => acc + v.usage, 0) /
        sysinfo.cpu.length,
      2
    );
    
    avgCpuDoughnutData.datasets[0].data[0] = avg;
    avgCpuDoughnutData.datasets[0].data[1] = 100 - avg;
    avgCpu = avg;
  }
</script>

<template>
  <div class="rounded-md p-3 text-center space-y-2 border-2 border-base01 h-full">
    <h3 class="text-xl font-bold">Average CPU Usage</h3>
    <Doughnut
      class="mx-2"
      data={avgCpuDoughnutData}
      options={defaultDoughnutOptions}
    />
    <p class="font-semibold text-sm">
      {avgCpu}%
    </p>
  </div>
</template>
