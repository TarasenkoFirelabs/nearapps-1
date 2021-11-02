import React from 'react';
import ReactDOM from 'react-dom';
import { transitions, positions, Provider as AlertProvider } from 'react-alert'
import App from './App';
import getConfig from './config.js';
import * as nearAPI from 'near-api-js';

// Initializing contract
async function initContract() {
  const nearConfig = getConfig(process.env.NODE_ENV || 'testnet');

  // Initializing connection to the NEAR TestNet
  const near = await nearAPI.connect({
    deps: {
      keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore()
    },
    ...nearConfig
  });

  // Needed to access wallet
  const walletConnection = new nearAPI.WalletConnection(near);

  // Load in account data
  let currentUser;
  if(walletConnection.getAccountId()) {
    currentUser = {
      accountId: walletConnection.getAccountId(),
      balance: (await walletConnection.account().state()).amount
    };
  }

  // Initializing our contract APIs by contract name and configuration
  const contract = await new nearAPI.Contract(walletConnection.account(), nearConfig.contractName, {
    // View methods are read-only – they don't modify the state, but usually return some value
    viewMethods: [],
    // Change methods can modify the state, but you don't receive the returned value when called
    changeMethods: ['log_analytics'],
    // Sender is the account ID to initialize transactions.
    // getAccountId() will return empty string if user is still unauthorized
    sender: walletConnection.getAccountId()
  });

  let account;
  if(walletConnection.getAccountId()) {
    account = await near.account(currentUser.accountId);
  }

  return { account, contract, currentUser, nearConfig, walletConnection };
}

window.nearInitPromise = initContract()
  .then(({ account, contract, currentUser, nearConfig, walletConnection }) => {

    // optional configuration
    const options = {
      position: positions.TOP_RIGHT,
      timeout: 5000,
      offset: '30px',
      transition: transitions.FADE
    }

    const AlertTemplate = ({ style, options, message, close }) => (
      <div style={style}>
        <button onClick={close}>X</button>
        <div>View this transaction in <a href={message} target="_blank">explorer</a></div>
      </div>
    )
    
    const Root = () => (
      <AlertProvider template={AlertTemplate} {...options}>
      <App
        account={account}
        contract={contract}
        currentUser={currentUser}
        nearConfig={nearConfig}
        wallet={walletConnection}
      />
      </AlertProvider>
    )

    ReactDOM.render(
      <Root />,
      document.getElementById('root')
    );
  });
