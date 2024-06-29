<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let token;

  onMount(() => {
    if (typeof window !== 'undefined') {
      token = localStorage.getItem('token');
      if (token) {
        goto('/protected');
      }
    }
  });

  function navigateToLogin() {
    goto('/login');
  }

  function navigateToRegister() {
    goto('/register');
  }

  function navigateToLink() {
    goto('/link');
  }
</script>

{#if typeof window === 'undefined' || !token}
  <h1>Welcome to the Home Screen</h1>
  <p>Please login or register to continue.</p>
  <button on:click={navigateToLogin}>Login</button>
  <button on:click={navigateToRegister}>Register</button>
  <button on:click={navigateToLink}>Link Accounts</button>
{:else}
  <h1>Welcome to the Protected Screen</h1>
  <p>This is a protected screen.</p>
{/if}
