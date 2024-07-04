<script>
    import { fade, fly } from 'svelte/transition';
    import { onMount } from 'svelte';
    import {
        getAllGroups,
        getDonationsForGroup,
        getDonationById,
        proposeDonation,
        approveDonation,
        getDonationProposalsForGroup,
        getDonationProposalById,
        rejectDonation,
        removeDonation,
    } from '$lib/services/api';
    import InstantiateContract from '$lib/components/InstantiateContract.svelte';


    let groups = [];
    let selectedGroup = null;
    let donations = [];
    let donation_proposals = [];
    let selectedDonation = null;
    let selectedDonationProposal = null;
    let error = '';
    let success = '';

    // Donation proposal form
    let proposal = {
        group_id: '',
        name: '',
        symbol: '',
        cost: '',
        description: '',
        image_url: ''
    };

    // Load all groups on mount
    onMount(async () => {
        try {
            groups = await getAllGroups();
        } catch (err) {
            console.error('Failed to load groups:', err);
            error = 'Failed to load groups';
        }
    });

    async function handleGroupChange(event) {
        selectedGroup = Number(event.target.value); // Convert the value to a number
        if (selectedGroup) {
            try {
                donations = await getDonationsForGroup(selectedGroup);
                donation_proposals = await getDonationProposalsForGroup(selectedGroup);
                selectedDonation = null;
                success = '';
                error = '';
            } catch (err) {
                console.error('Failed to load donations:', err);
                error = 'Failed to load donations';
                success = '';
            }
        }
    }

    async function handleDonationClick(id) {
        try {
            selectedDonation = await getDonationById(id);
            success = '';
            error = '';
        } catch (err) {
            console.error('Failed to load donation:', err);
            error = 'Failed to load donation';
            success = '';
        }
    }

    async function handleDonationProposalClick(id) {
        try {
            selectedDonationProposal = await getDonationProposalById(id);
            success = '';
            error = '';
        } catch (err) {
            console.error('Failed to load donation:', err);
            error = 'Failed to load donation';
            success = '';
        }
    }

    async function handleProposeDonation() {
        try {
            proposal.group_id = selectedGroup;
            await proposeDonation(proposal);
            success = 'Donation proposal submitted successfully';
            error = '';
            proposal = { group_id: '', name: '', symbol: '', cost: '', description: '', image_url: '' };
            donation_proposals = await getDonationProposalsForGroup(selectedGroup);
        } catch (err) {
            console.error('Failed to propose donation:', err);
            error = 'Failed to propose donation';
            success = '';
        }
    }

    async function handleApproveDonationProposal(id) {
        try {
            await approveDonation(id, selectedGroup);
            selectedDonationProposal = null;
            success = 'Donation approved successfully';
            error = '';
            donations = await getDonationsForGroup(selectedGroup); 
            donation_proposals = await getDonationProposalsForGroup(selectedGroup);
        } catch (err) {
            console.error('Failed to approve donation:', err);
            error = 'Failed to approve donation';
            success = '';
        }
    }

    async function handleRejectDonationProposal(id) {
        try {
            await rejectDonation(id, selectedGroup);
            selectedDonationProposal = null;
            success = 'Donation rejected successfully';
            error = '';
            donations = await getDonationsForGroup(selectedGroup);
            donation_proposals = await getDonationProposalsForGroup(selectedGroup);
        } catch (err) {
            console.error('Failed to approve donation:', err);
            error = 'Failed to approve donation';
            success = '';
        }
    }

    async function handleRemoveDonation(id) {
        try {
            await removeDonation(id, selectedGroup);
            selectedDonation = null;
            success = 'Donation approved successfully';
            error = '';
            donations = await getDonationsForGroup(selectedGroup);
            donation_proposals = await getDonationProposalsForGroup(selectedGroup);
        } catch (err) {
            console.error('Failed to approve donation:', err);
            error = 'Failed to approve donation';
            success = '';
        }
    }
</script>

{#if error}
    <p>{error}</p>
{:else if success}
    <p>{success}</p>
{/if}

<div>
    <h2>Select Group</h2>
    <select on:change={handleGroupChange}>
        <option value="">Select a group</option>
        {#each groups as group}
            <option value={group.id}>{group.name}</option>
        {/each}
    </select>
</div>

{#if selectedGroup}
    <div in:fly={{ y: 200, duration: 500 }} out:fade>
        <h2>Propose Donation</h2>
        <div>
            <label for="name">Name:</label>
            <input type="text" id="name" bind:value={proposal.name} required />
        </div>
        <div>
            <label for="name">Symbol:</label>
            <input type="text" id="symbol" bind:value={proposal.symbol} required />
        </div>
        <div>
            <label for="cost">Cost:</label>
            <input type="number" id="cost" bind:value={proposal.cost} required />
        </div>
        <div>
            <label for="image_url">Image URL:</label>
            <input type="url" id="image_url" bind:value={proposal.image_url} />
        </div>
        <div>
            <label for="description">Description:</label>
            <textarea id="description" bind:value={proposal.description} required></textarea>
        </div>
        <button on:click={handleProposeDonation}>Propose Donation</button>
    </div>

    <div in:fly={{ y: 200, duration: 1000 }} out:fade>
        <h2>Donation proposals for Group</h2>
        <ul>
            {#each donation_proposals as donation_proposal (donation_proposal.id)}
                <li>
                    <button on:click={() => handleDonationProposalClick(donation_proposal.id)}>{donation_proposal.name}</button>
                </li>
            {/each}
        </ul>
    </div>

    {#if selectedDonationProposal}
        <div in:fade out:fade>
            <h2 transition:fade >Donation Proposal Details</h2>
            <p>Name: {selectedDonationProposal.name}</p>
            <p>Symbol: {selectedDonationProposal.symbol}</p>
            <p>Cost: {selectedDonationProposal.cost}</p>
            <p>Description: {selectedDonationProposal.description}</p>
            {#if selectedDonationProposal.image_url}
                <p>Image: <img src={selectedDonationProposal.image_url} alt={selectedDonationProposal.name} /></p>
            {/if}
            <button on:click={() => handleApproveDonationProposal(selectedDonationProposal.id)}>Approve</button>
            <button on:click={() => handleRejectDonationProposal(selectedDonationProposal.id)}>Reject</button>
        </div>
    {/if}

    <div in:fly={{ y: 200, duration: 1500 }} out:fade>
        <h2>Donations for Group</h2>
        <ul>
            {#each donations as donation (donation.id)}
                <li>
                    <button on:click={() => handleDonationClick(donation.id)}>{donation.name}</button>
                </li>
            {/each}
        </ul>
    </div>

    {#if selectedDonation}
        <div in:fade out:fade>
            <h2>Donation Details</h2>
            {#if selectedDonation.image_url}
                <p>Image: <img src={selectedDonation.image_url} alt={selectedDonation.name} /></p>
            {/if}
            <p>Name: {selectedDonation.name}</p>
            <p>Symbol: {selectedDonation.symbol}</p>
            {#if selectedDonation.contract_address}
                <p>Cost: {selectedDonation.cost}</p>
                <p>Address: {selectedDonation.contract_address}</p>
            {:else}
                <InstantiateContract donation={selectedDonation} />
            {/if}
            <p>Description: {selectedDonation.description}</p>
            <button on:click={() => handleRemoveDonation(selectedDonation.id)}>Remove</button>
        </div>
    {/if}
{/if}


