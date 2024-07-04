<script>
  import { onMount } from 'svelte';
  import { getAllGroups, getGroupById, removeGroupById } from '$lib/services/api';
  import ManageMembers from '$lib/components/ManageMembers.svelte';
    import { goto } from '$app/navigation';

  let groups = [];
  let groupId = '';
  let group = {};
  let error = '';
  let success = '';

  // Load all group related info on mount
  onMount(async () => {
    try {
      groups = await getAllGroups();
    } catch (err) {
      console.error('Failed to load groups info:', err);
      error = 'Failed to load groups info';
    }
  });

  async function handleGetGroupById(id) {
    try {
      group = await getGroupById(id);
      success = '';
      error = '';
    } catch (err) {
      console.error('Error fetching group:', err);
      error = 'Error fetching group';
      success = '';
    }
  }

  async function handleRemoveGroupById(id) {
    try {
      await removeGroupById(id);
      success = 'Group removed successfully';
      error = '';
      // Reload groups
      groups = await getAllGroups();
    } catch (err) {
      console.error('Error removing group:', err);
      error = 'Error removing group';
      success = '';
    }
  }
</script>

{#if error}
  <p>{error}</p>
{/if}

{#if success}
  <p>{success}</p>
{/if}


<div>
  <h2>All Groups</h2>
  <ul>
    {#each groups as group (group.id)}
      <li> <button on:click={() => handleGetGroupById(group.id)}>{group.name}</button></li>
    {/each}
  </ul>
</div>

<div>
  <ul><li><button on:click={() => goto("/proposals")}>Propose a group</button></li></ul>
</div>


{#if group.id}
  {groupId = group.id}
  <div>
    <h2>Group Details</h2>
    <p>Name: <br> {group.name}</p>
    <p>Image URL: <br> {group.image_url}</p>
    <p>Cashout Wallet Address: <br> {group.cashout_wallet_address}</p>
    <p>Primary Color: <br> {group.primary_color}</p>
    <p>Secondary Color: <br> {group.secondary_color}</p>
    <p>About Us: <br> {group.about_us}</p>
    <button on:click={() => handleRemoveGroupById(group.id)}>Remove</button>

    <ManageMembers {groupId} />
  </div>
{/if}
