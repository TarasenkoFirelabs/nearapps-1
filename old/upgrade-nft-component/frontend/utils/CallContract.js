async function callContract(account, nearConfig, methodName, args) {
    const response = await account.functionCall({
      contractId: nearConfig.contractName,
      methodName: methodName,
      args: args
    });

    return response.transaction.hash;
  }

  function getNearExplorerTxLink(nearConfig, txid) {
    return `https://explorer.${nearConfig.networkId}.near.org/transactions/${txid}`
  }

  module.exports = callContract, getNearExplorerTxLink;