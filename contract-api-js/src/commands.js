#!/usr/bin/env node
const program = require("commander");
const { prompt } = require("inquirer");
const { deployAll } = require("./deploy");


program.version("1.0.0").alias("v").description("Near Apps Sdk")


const questions = [
  {
    type: "input",
    name: "answer",
    message: "Are u sure(y/n)?",
  },
];
program
  .command("deploy")
  .alias("d")
  .description("deploy contracts")
  .action(() => {
    prompt(questions).then((answers) => deployAll(answers));
  });

program.parse(process.argv);
