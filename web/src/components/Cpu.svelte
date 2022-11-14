<script lang="ts">
  export let sysinfo = null;
  let globalAvg: number = null;

  let roundOff = (num: number, places: number) => {
    const x = Math.pow(10, places);
    return Math.round(num * x) / x;
  };

  $: if (sysinfo.cpu) {
    globalAvg = roundOff((sysinfo.cpu.reduce((acc: number, v: any) => acc + v.usage, 0) / sysinfo.cpu.length), 2);
  }
</script>

<main>
  {#if sysinfo.cpu}
    <div
      class="w-full grid grid-cols-1 md:grid-cols-2 grid-rows-2 md:grid-rows-1 gap-2"
    >
      <div
        class="bg-slate-600 rounded-md shadow-md p-3 grid grid-cols-2 grid-rows-{sysinfo
          .cpu.length + 2}"
      >
        {#if globalAvg}
          <p>Global</p>
          <progress min="0" max="100" value={globalAvg} class="w-full" />
          <hr class="col-span-2 my-1">
        {/if}
        {#each sysinfo.cpu as cpu, i}
          <p>Cpu {i + 1}</p>
          <progress min="0" max="100" value={cpu.usage} class="w-full" />
        {/each}
      </div>
      <div class="bg-slate-600 rounded-md shadow-md p-3" />
    </div>
  {/if}
</main>
