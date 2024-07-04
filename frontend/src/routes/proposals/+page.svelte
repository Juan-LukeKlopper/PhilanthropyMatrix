<script>
    import { onMount } from 'svelte';
    import { addGroup, approveGroupProposalById, getAllGroupProposals, getProposedGroupById, rejectGroupProposalById } from '$lib/services/api';
    import ColorPicker from 'svelte-awesome-color-picker';
  
    let token = "";
    let group_proposals = [];
    let group_proposal = {};
    let error = '';
    let success = '';
    let newGroup = {
      name: '',
      image_url: '',
      cashout_wallet_address: '',
      primary_color: '',
      secondary_color: '',
      about_us: ''
    };
  
    onMount(async () => {
      try {
        group_proposals = await getAllGroupProposals();
      } catch (err) {
        console.error('Failed to load groups info:', err);
        error = 'Failed to load groups info';
      }
    });
  
    async function getGroupProposals() {
      group_proposals = await getAllGroupProposals();
    }

    async function handleAddGroup() {
      try {
        const addedGroup = await addGroup(newGroup);
        group_proposals.push(addedGroup); // Add the new group to the list
        success = 'Group added successfully';
        error = '';
        newGroup = { name: '', image_url: '', cashout_wallet_address: '', primary_color: '', secondary_color: '', about_us: '' }; // Reset form
        group_proposals = await getAllGroupProposals();
      } catch (err) {
        console.error('Error adding group:', err);
        error = 'Error adding group';
        success = '';
      }
    }

    async function handleGetProposedGroupById(id) {
      try {
        group_proposal = await getProposedGroupById(id);
        success = '';
        error = '';
      } catch (err) {
        console.error('Error fetching group:', err);
        error = 'Error fetching group';
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
  
  <!-- Form to propose a new group -->
  <div>
    <h2>Add Group Proposal</h2>
    <div>
      <label for="name">Name:</label>
      <br>
      <input type="text" id="name" bind:value={newGroup.name} required />
    </div>
    <div>
      <label for="image_url">Image URL:</label>
      <br>
      <input type="text" id="image_url" bind:value={newGroup.image_url} />
    </div>
    <div>
      <label for="cashout_wallet_address">Cashout Wallet Address:</label>
      <br>
      <input type="text" id="cashout_wallet_address" bind:value={newGroup.cashout_wallet_address} required />
    </div>
    <div>
      <label for="primary_color">Primary Color:</label>
      <br>
      <ColorPicker
	      bind:hex={newGroup.primary_color}
      />
    </div>
    <div>
      <label for="secondary_color">Secondary Color:</label>
      <br>
      <ColorPicker
	      bind:hex={newGroup.secondary_color}
      />
    </div>
    <div>
      <label for="about_us">About Us:</label>
      <textarea id="about_us" bind:value={newGroup.about_us}></textarea>
    </div>
    <button class="button-50"  on:click={handleAddGroup}>Add Proposal</button>
  </div>

  <!-- List all groups_proposals -->
  <div>
    <h2>All Proposed Groups</h2>
    <ul>
      {#each group_proposals as group_proposal (group_proposal.id)}
        <li> <button class="button-50"  on:click={() => handleGetProposedGroupById(group_proposal.id)}>{group_proposal.name}</button></li>
      {/each}
    </ul>
  </div>
  
  <!-- Display proposed group details -->
  {#if group_proposal.id}
    <div>
      <h2>Group Details</h2>
      <p>Name: <br> {group_proposal.name}</p>
      <p>Proposer: <br> {group_proposal.user_id}</p>
      <p>Image URL: <br> {group_proposal.image_url}</p>
      <p>Cashout Wallet Address: <br> {group_proposal.cashout_wallet_address}</p>
      <p>Primary Color: <br> {group_proposal.primary_color}</p>
      <p>Secondary Color: <br> {group_proposal.secondary_color}</p>
      <p>About Us: <br> {group_proposal.about_us}</p>
      <button class="button-50"  on:click={() => {approveGroupProposalById(group_proposal.id), getGroupProposals()}}>Approve</button>
      <button class="button-50"  on:click={() => {rejectGroupProposalById(group_proposal.id), getGroupProposals()}}>Decline</button>
    </div>
  {/if}


  <style>
    textarea {
      width: 100%;
      height: 200px;
    }
  </style>
  