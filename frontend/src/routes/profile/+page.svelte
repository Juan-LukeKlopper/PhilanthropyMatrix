<script>
  import { onMount } from 'svelte';
  import { getProfile, linkAccounts, loginWithKeplr } from '$lib/services/api';
  import { goto } from '$app/navigation';
  
  let profile = {};
  let error = '';
  let success = '';
  let username = '';
  let password = '';

  onMount(async () => {
    try {
      const token = localStorage.getItem('token');
      if (!token) {
        goto('/login');
        return;
      }
      profile = await getProfile(token);
    } catch (err) {
      console.error('Failed to load profile:', err);
      error = 'Failed to load profile';
    }
  });

  async function handleLinkKeplr() {
    try {
      const { address } = await loginWithKeplr();
      await linkAccounts(profile.username, profile.password, address);
      profile = await getProfile(localStorage.getItem('token')); // Reload profile to get updated data
      success = 'Keplr wallet linked successfully';
      error = '';
    } catch (err) {
      console.error('Error linking Keplr wallet:', err);
      error = 'Error linking Keplr wallet';
      success = '';
    }
  }

  async function handleSetWeb2Credentials() {
    try {
      await linkAccounts(username, password, profile.keplr_address);
      profile = await getProfile(localStorage.getItem('token')); // Reload profile to get updated data
      success = 'Web2 credentials set successfully';
      error = '';
    } catch (err) {
      console.error('Error setting Web2 credentials:', err);
      error = 'Error setting Web2 credentials';
      success = '';
    }
  }
</script>

{#if error}
  <p>{error}</p>
{:else if profile.username}
  <h1>Welcome, {profile.username}</h1>
  <p>Keplr Address: {profile.keplr_address}</p>
  {#if profile.username === profile.keplr_address}
    <!-- Show the form to set Web2 credentials -->
    <div>
      <h2>Set Web2 Credentials</h2>
      <div>
        <label for="username">Username:</label>
        <input type="text" id="username" bind:value={username} required />
      </div>
      <div>
        <label for="password">Password:</label>
        <input type="password" id="password" bind:value={password} required />
      </div>
      <button on:click={handleSetWeb2Credentials}>Set Web2 Credentials</button>
    </div>
  {:else}
    <!-- Show the form to link Keplr wallet -->
    <button on:click={handleLinkKeplr}>Link Keplr Wallet</button>
  {/if}
  {#if success}
    <p>{success}</p>
  {/if}
{/if}
