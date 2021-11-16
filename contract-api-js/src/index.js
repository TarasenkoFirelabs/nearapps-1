#!/usr/bin/env node
const chalk = require("chalk");

const deploy = (resp) => {
  console.log(resp)
  if (resp.answer == "y")
    console.log(chalk.bgGray("Deploying contracts... "));
};

module.exports = {
  deploy,
};
