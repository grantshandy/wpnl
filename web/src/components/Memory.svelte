<script lang="ts">
  import { Doughnut, Line } from "svelte-chartjs";
  import { defaultDoughnutOptions } from "../tools.ts";
  import {
    Chart as ChartJS,
    Tooltip,
    ArcElement,
    CategoryScale,
    LineElement,
    LinearScale,
    PointElement,
  } from "chart.js";
  import { formatFileSize } from "../tools";

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

  let memoryDoughnutData = {
    labels: ["Used Memory", "Free Memory"],
    datasets: [
      {
        data: [0, 0],
        backgroundColor: ["#F7464A", "#46BFBD"],
      },
    ],
  };

  let swapDoughnutData = {
    labels: ["Used Swap", "Free Swap"],
    datasets: [
      {
        data: [0, 0],
        backgroundColor: ["#F7464A", "#46BFBD"],
      },
    ],
  };

  let memoryMemory = {
    labels: generateLabels(500),
    datasets: [
      {
        label: "Memory",
        fill: true,
        lineTension: 0.3,
        pointRadius: 0,
        borderColor: "#F7464A",
        data: [],
      },
      {
        label: "Swap",
        fill: true,
        lineTension: 0.3,
        pointRadius: 0,
        borderColor: "#46BFBD",
        data: [],
      },
    ],
  };

  $: if (sysinfo) {
    memoryDoughnutData.datasets[0].data = [
      sysinfo.memory.usedMemory,
      sysinfo.memory.totalMemory - sysinfo.memory.usedMemory,
    ];

    swapDoughnutData.datasets[0].data = [
      sysinfo.swap.usedSwap,
      sysinfo.swap.totalSwap - sysinfo.swap.usedSwap,
    ];

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
  {#if sysinfo.memory && sysinfo.swap}
    <div class="w-full grid grid-cols-2 md:grid-cols-4 gap-2">
      <div class="bg-slate-600 rounded-md shadow-md p-3 text-center space-y-2">
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
      <div class="bg-slate-600 rounded-md shadow-md p-3 text-center space-y-2">
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
      <div
        class="bg-slate-600 rounded-md shadow-md p-3 text-center col-span-2"
      >
        <div class="w-full flow-root items-center align-middle px-1">
          <h3 class="float-left font-semibold text-lg">
            Memory and Swap History
          </h3>
          <div class="float-right h-full flex space-x-2">
            <div class="flex items-center space-x-1">
              <div class="w-4 h-4 rounded-full bg-[#F7464A]" />
              <p class="text-sm italic">Memory</p>
            </div>
            <div class="flex items-center space-x-1">
              <div class="w-4 h-4 rounded-full bg-[#46BFBD]" />
              <p class="text-sm italic">Swap</p>
            </div>
          </div>
        </div>
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
          }}
        />
      </div>
    </div>
  {/if}
</main>
