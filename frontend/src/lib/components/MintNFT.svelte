<script>
    import { SecretNetworkClient, MsgExecuteContract, coinFromString, coinsFromString, MsgSend } from 'secretjs';

  
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
  
    async function mintNFT(image, name) {
      try {
        console.log("1");
        const { secretjs, address } = await getSecretJs();

        console.log("2");
        console.log("donation: ",donation)
  
        const mintMsg = new MsgExecuteContract({
          sender: address,
          contract_address: donation.contract_address,
          code_hash: contractCodeHash,
          msg: {
            mint_nft: {
              owner: address,
              },
            },
        });


        const tx = await secretjs.tx.broadcast([mintMsg], {
          gasLimit: 200_000,
        });

        console.log("5");

  
        console.log('tx: ', tx);
        success = 'NFT minted successfully!';
        error = '';
      } catch (err) {
        console.error('Failed to mint NFT:', err);
        error = 'Failed to mint NFT';
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
  
  <button class="button-50"  on:click={() => mintNFT('https://example.com/image.png', 'Example NFT')}>Mint NFT</button>
  