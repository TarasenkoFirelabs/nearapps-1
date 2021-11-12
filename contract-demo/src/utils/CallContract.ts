const encode_utf8_base64 = (str) => {
  return Buffer.from(str).toString('base64');
}

async function callContract(account: any, nearConfig: any, methodName: string, args: {[key: string]: any}) {
  const response = await account.functionCall({
    contractId: nearConfig.contractName,
    methodName,
    args
  });

  console.log("response", response);

  return response.transaction.hash;
}

module.exports = callContract;