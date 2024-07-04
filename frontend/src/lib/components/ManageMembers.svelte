<script>
    import { onMount } from 'svelte';
    import {  listGroupMembers, addMember, removeMember  } from '$lib/services/api';
  
    export let groupId;
    
    let members = [];
    let newMember = {
      user_id: '',
      is_admin: false
    };
    let error = '';
    let success = '';
  
    onMount(async () => {
      await loadMembers();
    });
  
    async function loadMembers() {
      try {
       members = await listGroupMembers(groupId);
        error = '';
      } catch (err) {
        console.error('Failed to load group members:', err);
        error = 'Failed to load group members';
      }
    }
  
    async function handleAddMember() {
      try {
        await addMember(groupId, newMember.user_id, newMember.is_admin);
        await loadMembers();
        success = 'Member added successfully';
        error = '';
      } catch (err) {
        console.error('Error adding member:', err);
        error = 'Error adding member';
        success = '';
      }
    }
  
    async function handleRemoveMember(userId) {
      try {
        await removeMember(groupId, userId);
        await loadMembers();
        success = 'Member removed successfully';
        error = '';
      } catch (err) {
        console.error('Error removing member:', err);
        error = 'Error removing member';
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
    <h3>Group Members</h3>
    <ul>
      {#each members as member (member.user_id)}
        <li>
          {member.user_id} - {member.is_admin ? 'Admin' : 'Member'}
          <button class="button-50"  on:click={() => handleRemoveMember(member.user_id)}>Remove</button>
        </li>
      {/each}
    </ul>
  
    <h3>Add Member</h3>
    <div>
      <label for="user_id">User ID:</label>
      <input type="number" id="user_id" bind:value={newMember.user_id} required />
    </div>
    <div>
      <label for="is_admin">Is Admin:</label>
      <input type="checkbox" id="is_admin" bind:checked={newMember.is_admin} />
    </div>
    <button class="button-50"  on:click={handleAddMember}>Add Member</button>
  </div>
  