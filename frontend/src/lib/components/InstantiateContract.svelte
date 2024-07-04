<script>
    import { SecretNetworkClient, Wallet } from 'secretjs';
    import { linkDonationContract } from '$lib/services/api';

    export let donation = {};
  
    let contractAddress = '';
    let error = '';
    // Constant snip721 contract info MOOSE
    const codeId = import.meta.env.VITE_SNIP721_CODE_ID;
    const contractCodeHash = import.meta.env.VITE_SNIP721_CODE_HASH;
    const chainId = import.meta.env.VITE_CHAIN_ID;
    const API_URL = import.meta.env.VITE_CHAIN_URL;
  
    async function instantiateContract() {
      try {
        const wallet = window.keplr.getOfflineSignerOnlyAmino(chainId);
        const [{ address }] = await wallet.getAccounts();

        const secretjs = new SecretNetworkClient({
          chainId: chainId,
          url: API_URL,
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
  