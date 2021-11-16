const { keyStores, connect } = require("near-api-js");
const chalk = require("chalk");
const fs = require("fs");
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";
const ACCOUNT_ID = "near-example.testnet";
const WASM_PATH = path.join(__dirname, "/wasm-files/status_message.wasm");
const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

const config = {
  keyStore,
  networkId: "testnet",
  nodeUrl: "https://rpc.testnet.near.org",
};

async function deployContract(accountId, wasmPath) {
  const near = await connect(config);
  const account = await near.account(accountId);
  const result = await account.deployContract(fs.readFileSync(wasmPath));
  console.log(result);
}

const deployAll = (resp) => {
  console.log(resp);
  if (resp.answer == "y") {
    console.log(chalk.bgGray("Deploying contracts... "));
   // deployContract(ACCOUNT_ID, WASM_PATH);
  }
};

module.exports = {
  deployAll,
};