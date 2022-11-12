<script lang="ts">
  import Dashboard from "./components/Dashboard.svelte";
  import axios from "axios";

  let password: string = localStorage.getItem("password");
  let loggedin: boolean = false;

  if (password) {
    tryLogin();
  } else {
    password = "";
  }

  function tryLogin() {
    axios(`/auth/${password}`, { responseType: "text" })
      .then((response) => {
        if (response.data == "true") {
          loggedin = true;
          localStorage.setItem("password", password);
        } else {
          loggedin = false;
        }
      })
      .catch((_error) => {
        loggedin = false;
      });
  }
</script>

<main>
  <div class="w-full min-h-screen bg-slate-800 text-slate-200">
    {#if loggedin}
      <Dashboard />
    {:else}
      <div class="w-full min-h-screen grid place-items-center">
        <div
          class="space-y-4 p-6 bg-slate-700 rounded-md shadow-md text-center"
        >
          <h1 class="text-3xl font-bold">Admin Login</h1>
          <div class="flex space-x-2">
            <input
              class="p-2 rounded-md text-slate-900"
              type="Password"
              placeholder="password"
              bind:value={password}
            />
            <button
              class="p-2 font-bold bg-slate-600 rounded-md shadow-md"
              on:click={tryLogin}>Login</button
            >
          </div>
        </div>
      </div>
    {/if}
  </div>
</main>
