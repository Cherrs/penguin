<script>
  import Login from "./lib/Login/Login.svelte";
  import Main from "./lib/Main.svelte";
  import { is_login } from "./store";
</script>

<main>
  {#if $is_login}
    <Main />
  {:else}
    <Login />
  {/if}
</main>

<style>
</style>
