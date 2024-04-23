const anchor = require('@project-serum/anchor');
const fs = require('fs');

(async () => {
    const provider = anchor.Provider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.SolanaMemeCoin;
    
    const holdersRawData = fs.readFileSync('holders.json');
    const holders = JSON.parse(holdersRawData);
    
    for (const holder of holders) {
        await program.rpc.airdropTokens(new anchor.BN(1000), {
            accounts: {
                memeCoinMint: program.memeCoinMint,
                authority: provider.wallet.publicKey,
                holderAccounts: holder.address, // Assuming 'address' is the field in JSON
                tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
            },
        });
    }
})();
