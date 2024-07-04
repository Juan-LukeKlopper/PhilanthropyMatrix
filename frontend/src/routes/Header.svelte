<script>
	import { goto } from '$app/navigation';
  
	function navigateToProfile() {
	  goto('/profile');
	}
  
	function navigateToLogin() {
	  goto('/login');
	}
  
	function navigateToRegister() {
	  goto('/register');
	}
  
	export let isLoggedIn = false;
  
	function logout() {
	  localStorage.removeItem('token');
	  window.location.href = '/login';
	}
  </script>
  
  <nav>
	<a href="/">Home</a>
	{#if isLoggedIn}
	  <a href="/profile">Profile</a>
	  <button class="button-50"  on:click={logout}>Logout</button>
	{:else}
	  <a href="/login">Login</a>
	  <a href="/register">Register</a>
	{/if}
  </nav>
  