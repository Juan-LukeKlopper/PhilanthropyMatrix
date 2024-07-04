<script>
    import { SecretNetworkClient, Wallet } from 'secretjs';
    import { linkDonationContract } from '$lib/services/api';

    export let donation = {};
  
    let contractAddress = '';
    let error = '';
    // Constant snip721 contract info
    const codeId = "9104";
    const contractCodeHash = "f52c670d1cf2f64b93197b5c6c8e61af1d228e92b7d133fd02c097f34b37b3bb";
  
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
          config: { 
            public_token_supply: true, 
            minter_may_update_metadata: false,
          },
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
  