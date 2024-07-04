<script>
    import { SecretNetworkClient, Wallet } from 'secretjs';
    import { linkDonationContract } from '$lib/services/api';

    export let donation = {};
  
    let contractAddress = '';
    let error = '';
    // Constant snip721 contract info
    const codeId = "9070";
    const contractCodeHash = "773c39a4b75d87c4d04b6cfe16d32cd5136271447e231b342f7467177c363ca8";
  
    async function instantiateContract() {
      try {
        const wallet = window.keplr.getOfflineSignerOnlyAmino('pulsar-3');
        const [{ address }] = await wallet.getAccounts();

        const secretjs = new SecretNetworkClient({
          chainId: 'pulsar-3',
          url: 'https://api.pulsar.scrttestnet.com',
          wallet: wallet,
          walletAddress: address,
        });

        const initMsg = {
          name: donation.name,
          symbol: donation.symbol,
          entropy: 'qwertyuioplkjhgfdsa',
        };

        const tx = await secretjs.tx.compute.instantiateContract(
          {
            code_id: codeId,
            sender: address,
            code_hash: contractCodeHash,
            init_msg: initMsg,
            label:  donation.name,
          },
          {
            gasLimit: 400_000,
          }
        );

        contractAddress = tx.arrayLog.find(
          (log) => log.type === 'message' && log.key === 'contract_address'
        ).value;

        await linkDonationContract(donation.group_id, donation.id, contractAddress);
        
        success = 'Contract instantiated and linked successfully!';
        error = '';
      } catch (err) {
        console.error('Error instantiating contract:', err);
        error = 'Failed to instantiate contract';
      }
    }
  
    async function handleInstantiateContract() {
      await instantiateContract();
    }
  </script>
  
  <main>
      {#if donation}
        {#if contractAddress}
          <p>Contract Address: {contractAddress}</p>
          <p>Cost: {donation.cost}</p>
        {:else}
            <button on:click={handleInstantiateContract}>Instantiate Contract</button>
        {/if}
      {:else}
        <p>Loading donation details...</p>
      {/if}
  </main>
  