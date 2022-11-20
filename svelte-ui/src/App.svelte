<script>
  import Login from "./lib/Login/Login.svelte";
  import Main from "./lib/Main.svelte";

  let is_login = false;
</script>

<main>
  {#if !is_login}
    <Login />
  {:else}
    <Main />
  {/if}
</main>

<style>
</style>
