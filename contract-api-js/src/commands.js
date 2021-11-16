#!/usr/bin/env node
const program = require("commander");
const { prompt } = require("inquirer");
const { createAccount } = require("near-api-js/lib/transaction");
const { deployAll } = require("./deploy");
const { createWallet } = require("./accounts");

program.version("1.0.0").alias("v").description("Near Apps Api-Js")


program
  .command("deploy")
  .alias("d")
  .description("deploy contracts")
  .action(() => {
    prompt([
      {
        type: "input",
        name: "answer",
        message: "Are u sure(y/n)?",
      },
    ]).then((answers) => deployAll(answers));
  });

  program
    .command("wallet")
    .alias("w")
    .description("create wallet")
    .action(() => {
      prompt([
        {
          type: "input",
          name: "wallet",
          message: "Wallet Name",
        },
      ]).then((answers) => createWallet(answers));
    });

program.parse(process.argv);
