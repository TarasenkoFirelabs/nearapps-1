import 'regenerator-runtime/runtime';
import React, { useState, useEffect } from 'react';
import { useAlert } from 'react-alert'
import PropTypes from 'prop-types';
import Form from './components/Form';
import SignIn from './components/SignIn';
import callContract from './utils/CallContract';

const App = ({ account, contract, currentUser, nearConfig, wallet }) => {
  const alert = useAlert();

  let initData = {
      app_id: 'Example App Log',
      action_id: 'Example Action Log',
      user_name: currentUser ? currentUser.accountId : 'Log SuperHero'
    };

  const [analytics, setAnalytics] = useState(initData);
  const [isSubmitting, setIsSubmitting] = useState(false)

  useEffect(() => {
    // 
  }, []);

  function encode_utf8_base64(str) {
    return Buffer.from(str).toString('base64');
  }

  const encode64 = (formData) => {
    return encode_utf8_base64(formData.app_id) + '_' +
      encode_utf8_base64(formData.action_id) + '_' +
      encode_utf8_base64(formData.user_name);
  }

  const onSubmit = (formData) => {
    setIsSubmitting(true);

    // either you can call `contract.log_analytics(args);`
    // but this will not return TransactionId, cause doesn't specified in contract `contract\src\lib.rs`

    try {
      const args = {"encoded": encode64( formData )};
      callContract(account, nearConfig, 'log_analytics', args)
        .then((txid) => {
          setIsSubmitting(false)

          let link = `https://explorer.${nearConfig.networkId}.near.org/transactions/${txid}`;
          alert.show(link);
        })

    } catch(error) {
      alert.error(error);
    }
  };

  const signIn = () => {
    wallet.requestSignIn(
      nearConfig.contractName,
      'NEAR Analytics Log'
    );
  };

  const signOut = () => {
    wallet.signOut();
    window.location.replace(window.location.origin + window.location.pathname);
  };

  return (
    <main>
      <header>
        <h1>NEAR Log Analytics</h1>
        { currentUser
          ? <button onClick={signOut}>Log out</button>
          : <button onClick={signIn}>Log in</button>
        }
      </header>
      { currentUser
        ? <Form onSubmit={onSubmit} currentUser={currentUser} analytics={analytics} disabled={isSubmitting} />
        : <SignIn/>
      }
    </main>
  );
};

App.propTypes = {
  contract: PropTypes.shape({
    log_analytics: PropTypes.func.isRequired
  }).isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    balance: PropTypes.string.isRequired
  }),
  analytics: PropTypes.shape({
    app_id: PropTypes.string.isRequired,
    action_id: PropTypes.string.isRequired,
    user_name: PropTypes.string.isRequired
  }),
  nearConfig: PropTypes.shape({
    contractName: PropTypes.string.isRequired
  }).isRequired,
  wallet: PropTypes.shape({
    requestSignIn: PropTypes.func.isRequired,
    signOut: PropTypes.func.isRequired
  }).isRequired
};

export default App;
