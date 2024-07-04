<script>
    import { SecretNetworkClient, MsgExecuteContract, coinFromString } from 'secretjs';
  
    export let donation;
    const contractCodeHash = import.meta.env.VITE_SNIP721_CODE_HASH;
    const chainId = import.meta.env.VITE_CHAIN_ID;
    const API_URL = import.meta.env.VITE_CHAIN_URL;
    let error = '';
    let success = '';
  
    // Function to initialize the SecretJS client using Keplr
    async function getSecretJs() {
      if (!window.keplr) {
        throw new Error('Please install Keplr extension');
      }
  
      await window.keplr.enable(chainId);
      const keplrOfflineSigner = window.getOfflineSigner(chainId);
      const [{ address }] = await keplrOfflineSigner.getAccounts();
  
      const secretjs = new SecretNetworkClient({
        chainId: chainId,
        url: API_URL,
        wallet: keplrOfflineSigner,
        walletAddress: address,
      });
  
      return { secretjs, address };
    }
  
    async function addMinter() {
      try {
        const { secretjs, address } = await getSecretJs();
  
        const addMinterMsg = new MsgExecuteContract({
          sender: address,
          contract_address: donation.contract_address,
          code_hash: contractCodeHash,
          msg: {
            add_minter: {
              minter: address,
            },
          },
          sent_funds: [coinFromString('1000000uscrt')], // Adjust the amount as needed
        });
  
        const tx = await secretjs.tx.broadcast([addMinterMsg], {
          gasLimit: 200_000,
        });
  
        console.log('tx: ', tx);
        success = 'Minter added successfully!';
        error = '';
      } catch (err) {
        console.error('Failed to add minter:', err);
        error = 'Failed to add minter';
        success = '';
      }
    }
  </script>
  
  <style>
    .error {
      color: red;
    }
    .success {
      color: green;
    }
  </style>
  
  {#if error}
    <p class="error">{error}</p>
  {/if}
  
  {#if success}
    <p class="success">{success}</p>
  {/if}
  
  <button class="button-50"  on:click={addMinter}>Add Minter</button>
  