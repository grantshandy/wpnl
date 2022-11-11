<script lang="ts">
  console.log("starting");

  let eventSource = new EventSource("/sysinfo-broadcast");

  eventSource.onmessage = (event) => {
    let sysInfo = JSON.parse(event.data);
    
    console.log(sysInfo);
  };
</script>

<main>
  <h1 class="text-2xl font-bold">Hello, World!</h1>
  <p>some text</p>
</main>
