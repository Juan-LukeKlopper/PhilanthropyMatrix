<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let token;

  onMount(() => {
    if (typeof window !== 'undefined') {
      token = localStorage.getItem('token');
      if (!token) {
        goto('/');
      }
    }
  });
</script>

<h1>Welcome to the Protected Screen</h1>
<p>This is a protected screen.</p>

