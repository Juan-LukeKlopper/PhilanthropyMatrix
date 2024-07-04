<script>
  import { onMount } from 'svelte';
  import { getProfile, changeCredentials, addWallet, loginWithKeplr } from '$lib/services/api';
  import { goto } from '$app/navigation';
  import Minidenticon from '$lib/components/Minidenticons.svelte';
  
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
      profile = await getProfile();
    } catch (err) {
      console.error('Failed to load profile:', err);
      error = 'Failed to load profile';
    }
  });

  async function handleChangeCredentials() {
    try {
      await changeCredentials(profile.keplr_address, username, password);
      profile = await getProfile(); // Reload profile to get updated data
      success = 'Credentials updated successfully';
      error = '';
    } catch (err) {
      console.error('Error changing credentials:', err);
      error = err.message;
      success = '';
    }
  }

  async function handleAddWallet() {
    try {
      await addWallet(); // Ensure password is sent
      profile = await getProfile(); // Reload profile to get updated data
      success = 'Wallet address added successfully';
      error = '';
    } catch (err) {
      console.error('Error adding wallet address:', err);
      error = err.message;
      success = '';
    }
  }
</script>

{#if error}
  <p>{error}</p>
{:else if profile.username}
  <Minidenticon username={profile.username} />
  <h1>Welcome, {profile.username}</h1>
  <p>Keplr Address: {profile.keplr_address}</p>
  {#if !profile.keplr_address}
    <!-- Show the form to add wallet address -->
    <button on:click={handleAddWallet}>Link Keplr Wallet</button>
  {:else if profile.username === profile.keplr_address}
    <!-- Show the form to change username and password -->
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
      <button on:click={handleChangeCredentials}>Set Web2 Credentials</button>
    </div>
  {/if}
  {#if success}
    <p>{success}</p>
  {/if}
{/if}