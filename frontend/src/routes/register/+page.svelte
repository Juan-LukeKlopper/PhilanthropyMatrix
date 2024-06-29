<script>
  import { register, loginWithKeplr } from '$lib/services/api';
  import { goto } from '$app/navigation';
  let username = '';
  let password = '';
  let error = '';

  async function handleRegister() {
    console.log('Register form submitted with username:', username);
    try {
      const response = await register({ username, password });
      console.log('Register response:', response);
      error = '';
      goto('/login');
    } catch (err) {
      console.error('Error during registration:', err);
      error = 'Registration failed';
    }
  }

  async function handleKeplrRegister() {
    try {
      const response = await loginWithKeplr();
      console.log('Keplr registration response:', response);
      error = '';
      goto('/protected');
    } catch (err) {
      console.error('Error during Keplr registration:', err);
      error = 'Keplr registration failed';
    }
  }
</script>

<form on:submit|preventDefault={handleRegister}>
  <div>
    <label for="username">Username:</label>
    <input type="text" id="username" bind:value={username} required />
  </div>
  <div>
    <label for="password">Password:</label>
    <input type="password" id="password" bind:value={password} required />
  </div>
  <button type="submit">Register</button>
  <button type="button" on:click={handleKeplrRegister}>Register with Keplr</button>
  {#if error}
    <p>{error}</p>
  {/if}
</form>
