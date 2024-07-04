<script>
    import { SecretNetworkClient, MsgExecuteContract, coinFromString } from 'secretjs';
  
    export let donation;
    const contractCodeHash = "f52c670d1cf2f64b93197b5c6c8e61af1d228e92b7d133fd02c097f34b37b3bb";
    let error = '';
    let success = '';
  
    // Function to initialize the SecretJS client using Keplr
    async function getSecretJs() {
      if (!window.keplr) {
        throw new Error('Please install Keplr extension');
      }
  
      await window.keplr.enable('pulsar-3');
      const keplrOfflineSigner = window.getOfflineSigner('pulsar-3');
      const [{ address }] = await keplrOfflineSigner.getAccounts();
  
      const secretjs = new SecretNetworkClient({
        chainId: 'pulsar-3',
        url: 'https://api.pulsar3.scrttestnet.com',
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
  
  <button on:click={addMinter}>Add Minter</button>
  