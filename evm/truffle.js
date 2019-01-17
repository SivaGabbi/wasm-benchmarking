/*
 * NB: since truffle-hdwallet-provider 0.0.5 you must wrap HDWallet providers in a 
 * function when declaring them. Failure to do so will cause commands to hang. ex:
 * ```
 * mainnet: {
 *     provider: function() { 
 *       return new HDWalletProvider(mnemonic, 'https://mainnet.infura.io/<infura-key>') 
 *     },
 *     network_id: '1',
 *     gas: 4500000,
 *     gasPrice: 10000000000,
 *   },
 */

module.exports = {
    networks: {
      development: {
        host: "127.0.0.1",
        port: 8545,
        network_id: "*",
        from: "0x1cD92B2DFEf65A55d5F6Cd741Ca6d8c819E46C12",
        gas: 4500000,
        gasPrice: 10000000000 // Match any network id
      }
    },
    compilers: {
      solc: {
        version: "0.4.24" // ex:  "0.4.20". (Default: Truffle's installed solc)
      }
   }
  
};
