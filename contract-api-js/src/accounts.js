const chalk = require("chalk");

const createWallet = (resp) => {
  console.log(chalk.greenBright(`Wallet ${resp.wallet} has been successfully created... `));
};

module.exports = {
  createWallet,
};
