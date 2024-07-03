<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  let isLoggedIn = false;

  onMount(() => {
    const token = localStorage.getItem('token');
    isLoggedIn = !!token;
  });

  function logout() {
    localStorage.removeItem('token');
    isLoggedIn = false;
    goto('/login');
  }
</script>

<nav>
  <a href="/">Home</a>
  {#if isLoggedIn}
    <a href="/profile">Profile</a>
    <a href="/groups">Groups</a>
    <button on:click={logout}>Logout</button>
  {:else}
    <a href="/login">Login</a>
    <a href="/register">Register</a>
  {/if}
</nav>

<slot />
