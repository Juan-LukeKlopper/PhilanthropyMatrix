<script>
    export let donations = [];
    
    function handleMint(donation) {
        console.log("hello")
    }
  </script>
  
  <style>
    .donation-item {
      border: 1px solid #ddd;
      padding: 10px;
      margin-bottom: 10px;
      border-radius: 5px;
      background-color: #f9f9f9;
    }
  
    .donation-item.disabled {
      background-color: #e0e0e0;
      cursor: not-allowed;
    }
  
    .donation-item.enabled {
      background-color: #ffffff;
      cursor: pointer;
    }
  
    .mint-button {
      padding: 5px 10px;
      background-color: #4CAF50;
      color: white;
      border: none;
      border-radius: 3px;
      cursor: pointer;
    }
  
    .mint-button:disabled {
      background-color: #e0e0e0;
      cursor: not-allowed;
    }
  </style>
  
  <div>
    {#each donations as donation}
      <div class="donation-item {donation.contract_address ? 'enabled' : 'disabled'}">
        <p><strong>Name:</strong> {donation.name}</p>
        <p><strong>Cost:</strong> {donation.cost}</p>
        <p><strong>Description:</strong> {donation.description}</p>
        <p><strong>Image URL:</strong> {donation.image_url}</p>
        <button 
          class="mint-button" 
          on:click={() => handleMint(donation)} 
          disabled={!donation.contract_address}>
          Mint Donation
        </button>
      </div>
    {/each}
  </div>
  