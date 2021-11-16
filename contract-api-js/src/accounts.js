const chalk = require("chalk");
const { keyStores, connect, Contract } = require("near-api-js");
const { generateSeedPhrase } = require("near-seed-phrase");
const fs = require("fs");
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";
const ACCOUNT_ID = "dev-1636986913935-73551190344227";
const MAKE_WALLET_CONTRACT_ID = "adsick.testnet";
const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);



const config = {
    keyStore,
    networkId: "testnet",
    nodeUrl: "https://rpc.testnet.near.org",
};

async function callCreateWallet(accountId, publicKey) {
    const near = await connect(config);
    const account = await near.account(ACCOUNT_ID);
    const contract = new Contract(account, MAKE_WALLET_CONTRACT_ID, {
        // name of contract you're connecting to
        viewMethods: [], // view methods do not change state but usually return a value
        changeMethods: ["make_wallet"], // change methods modify state
        sender: account, // account object to initialize and sign transactions.
    });

    const result = await contract.make_wallet(
        {
            new_account: { "account_id": accountId, "public_key": publicKey }, // argument name and value - pass empty object if no args required

            gas: 300000000000000, // attached GAS (optional)
            amount: 1000000000000000000000000 // attached deposit in yoctoNEAR (optional)
        }
    );

    console.log(result);
}


const createWallet = (resp) => {

    //TODO: Seed phrase generation here
    const { seedPhrase, publicKey } = generateSeedPhrase();
    console.log(chalk.gray(`SeedPhrase:`) + chalk.bgGray(seedPhrase))
    callCreateWallet(resp.wallet, publicKey);

    console.log(chalk.greenBright(`Wallet ${resp.wallet} has been successfully created... `));
};

async function approveId(accountId, contractId) {
    const near = await connect(config);
    const account = await near.account(accountId);
    const contract = new Contract(account, accountId, {
      viewMethods: [],
      changeMethods: ["add_contract"],
      sender: account,
    });
    const result = await contract.add_contract({contract_name: contractId.accountId});
    console.log(result);
  }
  
  const approveContract = (resp) => {
    approveId(ACCOUNT_ID, resp);
  }  

module.exports = {
    createWallet,
    approveContract,
};
