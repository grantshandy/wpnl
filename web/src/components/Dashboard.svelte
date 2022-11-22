<script lang="ts">
  import Resources from "./Resources.svelte";
  import Info from "./Info.svelte";

  enum PageState {
    Resources = "Resources",
    Info = "Info",
  }

  let pages = [PageState.Resources, PageState.Info];

  let currentpage: PageState = PageState.Resources;

  function logOut() {
    localStorage.removeItem("password");
    window.location.reload();
  }
</script>

<main>
  <div class="w-full min-h-screen">
    <div class="w-5/6 md:w-2/3 lg:w-1/2 min-h-screen mx-auto py-3 space-y-3">
      <div class="w-full flex">
        <h1 class="grow text-3xl font-bold">Admin Panel</h1>
        <button
          class="bg-base02 rounded-md shadow-md hover:shadow-lg px-2 py-1 font-bold"
          on:click={() => logOut()}>Log Out</button
        >
      </div>
      <div>
        <div class="mx-2 inline-block">
          {#each pages as page}
            {#if page == currentpage}
              <button
                class="px-2 py-1 font-bold bg-base02 rounded-tr-md rounded-tl-md"
                >{page}</button
              >
            {:else}
              <button
                class="px-2 py-1 font-bold"
                on:click={() => (currentpage = page)}>{page}</button
              >
            {/if}
          {/each}
        </div>
        <div class="w-full bg-base02 rounded-md shadow-md p-3">
          {#if currentpage == "Resources"}
            <Resources />
          {:else if currentpage == "Info"}
            <Info />
          {/if}
        </div>
      </div>
    </div>
  </div>
</main>
