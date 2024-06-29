<script>
  import { linkAccounts } from '$lib/services/api';
  let username = '';
  let password = '';
  let keplrAddress = '';
  let error = '';
  let success = '';

  async function handleLinkAccounts() {
    try {
      const response = await linkAccounts(username, password, keplrAddress);
      console.log('Link accounts response:', response);
      success = 'Accounts linked successfully';
      error = '';
    } catch (err) {
      console.error('Error during linking accounts:', err);
      error = 'Linking accounts failed';
      success = '';
    }
  }
</script>

<form on:submit|preventDefault={handleLinkAccounts}>
  <div>
    <label for="username">Username:</label>
    <input type="text" id="username" bind:value={username} required />
  </div>
  <div>
    <label for="password">Password:</label>
    <input type="password" id="password" bind:value={password} required />
  </div>
  <div>
    <label for="keplrAddress">Keplr Address:</label>
    <input type="text" id="keplrAddress" bind:value={keplrAddress} required />
  </div>
  <button type="submit">Link Accounts</button>
  {#if error}
    <p>{error}</p>
  {/if}
  {#if success}
    <p>{success}</p>
  {/if}
</form>
