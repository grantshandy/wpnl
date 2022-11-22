<script lang="ts">
  import { Line } from "svelte-chartjs";
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


  const secondsChartBuffer: number = 60;

  function generateLabels(tickLength: number): string[] {
    let intervals = Math.round((secondsChartBuffer * 1000) / tickLength);

    let labels = [];

    for (var i = 0; i < intervals; i++) {
      labels.push("");
    }

    return labels;
  }

  let memoryMemory = {
    labels: generateLabels(500),
    datasets: [
      {
        label: "Memory",
        fill: true,
        lineTension: 0.3,
        pointRadius: 0,
        borderColor: chartColors[0],
        data: [],
      },
      {
        label: "Swap",
        fill: true,
        lineTension: 0.3,
        pointRadius: 0,
        borderColor: chartColors[1],
        data: [],
      },
    ],
  };

  $: if (sysinfo) {
    memoryMemory.labels = generateLabels(sysinfo.tickLength);
	
	
    if (
      memoryMemory.datasets[0].data.length >
      Math.round((secondsChartBuffer * 1000) / sysinfo.tickLength)
    ) {
      memoryMemory.datasets[0].data.shift();
      memoryMemory.datasets[1].data.shift();
    }

    memoryMemory.datasets[0].data.push(
      ((sysinfo.memory.usedMemory / sysinfo.memory.totalMemory) * 100).toFixed(
        2
      )
    );
    memoryMemory.datasets[1].data.push(
      ((sysinfo.swap.usedSwap / sysinfo.swap.totalSwap) * 100).toFixed(2)
    );
	
}
</script>

<main>
  <div class="rounded-md p-3 text-center border-2 border-base01 col-span-2 h-full">
    <h3 class="font-semibold text-xl">Memory and Swap History</h3>
    <Line
      class="w-full h-full"
      data={memoryMemory}
      options={{
        animation: false,
        scales: {
          yAxis: {
            suggestedMin: 0,
            suggestedMax: 100,
          },
        },
        borderColor: "#859900",
      }}
    />
    <div class="w-full flow-root">
      <div class="float-right h-full flex space-x-2">
        <div class="flex items-center space-x-1">
          <div class="w-4 h-4 rounded-full" style="background-color: {chartColors[0]}" />
          <p class="text-sm italic font-bold">Memory</p>
        </div>
        <div class="flex items-center space-x-1">
          <div class="w-4 h-4 rounded-full" style="background-color: {chartColors[1]}" />
          <p class="text-sm italic font-bold">Swap</p>
        </div>
      </div>
    </div>
  </div>
</main>
