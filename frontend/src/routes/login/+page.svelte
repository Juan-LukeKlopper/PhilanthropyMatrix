<script>
  import { login, loginWithKeplr } from '$lib/services/api';
  import { goto } from '$app/navigation';
  let username = '';
  let password = '';
  let error = '';

  async function handleLogin() {
    console.log('Login form submitted with username:', username);
    try {
      const response = await login({ username, password });
      console.log('Login response:', response);
      localStorage.setItem('token', response.token);
      error = '';
      goto('/protected');
    } catch (err) {
      console.error('Error during login:', err);
      error = 'Login failed';
    }
  }

  async function handleKeplrLogin() {
    try {
      const response = await loginWithKeplr();
      console.log('Keplr login response:', response);
      error = '';
      goto('/protected');
    } catch (err) {
      console.error('Error during Keplr login:', err);
      error = 'Keplr login failed';
    }
  }
</script>

<form on:submit|preventDefault={handleLogin}>
  <div>
    <label for="username">Username:</label>
    <input type="text" id="username" bind:value={username} required />
  </div>
  <div>
    <label for="password">Password:</label>
    <input type="password" id="password" bind:value={password} required />
  </div>
  <button type="submit">Login</button>
  <button type="button" on:click={handleKeplrLogin}>Login with Keplr</button>
  {#if error}
    <p>{error}</p>
  {/if}
</form>
