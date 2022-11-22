<script lang="ts">
  import { Line } from "svelte-chartjs";
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
  const secondsChartBuffer: number = 60;

  function generateLabels(tickLength: number): string[] {
    let intervals = Math.round((secondsChartBuffer * 1000) / tickLength);

    let labels = [];

    for (var i = 0; i < intervals; i++) {
      labels.push("");
    }

    return labels;
  }

  let roundOff = (num: number, places: number) => {
    const x = Math.pow(10, places);
    return Math.round(num * x) / x;
  };

  let gapTick: boolean = false;

  let cpuMemory = {
    labels: generateLabels(500),
    datasets: [],
  };
	
	$: if (sysinfo) {
    if (cpuMemory.datasets.length != sysinfo.cpu.length) {
      sysinfo.cpu.forEach((_cpu: { usage: number }, index: number) => {
        cpuMemory.datasets.push({
          label: `Cpu ${index + 1}`,
          fill: true,
          lineTension: 0.3,
          pointRadius: 0,
          borderColor: chartColors[index],
          borderWidth: 0.8,
          data: [],
        });
      });
    }

    cpuMemory.labels = generateLabels(sysinfo.tickLength);

    cpuMemory.datasets.forEach((dataset) => {
      if (
        dataset.data.length >
        Math.round((secondsChartBuffer * 1000) / sysinfo.tickLength)
      ) {
        dataset.data.shift();
      }
    });

    for (let i = 0; i < cpuMemory.datasets.length; i++) {
      cpuMemory.datasets[i].data.push(sysinfo.cpu[i].usage);
    }
		
    if (gapTick) {
      gapTick = false;
    } else {
      gapTick = true;
    }
	}
</script>

<template>
  <div class="col-span-2 p-3 text-center rounded-md border-2 border-base01 h-full">
    <h3 class="text-xl font-bold">CPU History</h3>
    <Line
      class="w-full h-full"
      data={cpuMemory}
      options={{
        animation: false,
        scales: { yAxis: { suggestedMin: 0, suggestedMax: 100 } },
        borderColor: "#859900",
      }}
    />
  </div>
</template>
