async function callContract(account, nearConfig, methodName, args) {
    const response = await account.functionCall({
      contractId: nearConfig.contractName,
      methodName: methodName,
      args: args
    });

    return response.transaction.hash;
  }

  module.exports = callContract;