<script>
  import { onMount } from 'svelte';
  import Donations from '$lib/components/Donations.svelte';
  import { getDonations } from '$lib/services/api';


  let donations = [];
  let error = '';
  let token;
  let logged_in = false;

  onMount(async () => {
    try {
      if (typeof window !== 'undefined') {
        token = localStorage.getItem('token');
        if (token) {
          logged_in = true
        } 
      }
      donations = await getDonations();
    } catch (err) {
      console.error('Failed to load donations:', err);
      error = 'Failed to load donations';
    }
  });
</script>

{#if error}
  <p>{error}</p>
{/if}

<Donations {donations} />

<p>Please refresh to see new nav links!</p>
