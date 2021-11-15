import getConfig from "./config.js";

function getContractsConfig(env) {
  const Config = getConfig(env);
  const dataContracts = {
    GAS: "200000000000000",
    ATTACHED_DEPOSIT_AMOUNT: "1000000000000000000000000", /// 1 Ⓝ
    CONTRACT_NAME_MAKE_WALLET: process.env.CONTRACT_NAME_MAKE_WALLET,
    CONTRACT_NAME_NFT: process.env.CONTRACT_NAME_NFT
  };

  return {...Config, ...dataContracts}
}

module.exports = getContractsConfig;