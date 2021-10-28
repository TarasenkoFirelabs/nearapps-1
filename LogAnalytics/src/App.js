import 'regenerator-runtime/runtime';
import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';
import Form from './components/Form';
import SignIn from './components/SignIn';

const App = ({ contract, currentUser, nearConfig, wallet }) => {
  let initData = {
      app_id: 'Example App Log',
      action_id: 'Example Action Log',
      user_name: currentUser ? currentUser.accountId : 'Log SuperHero'
    };

  const [analytics, setAnalytics] = useState(initData);
  const [isSubmitting, setIsSubmitting] = useState(false)

  useEffect(() => {
    // TODO: don't just fetch once; subscribe!
    contract.get_analytics().then(setAnalytics)
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
    setIsSubmitting(true)

    // TODO: optimistically update page with new message,
    // update blockchain data in background
    // add uuid to each message, so we know which one is already known
    contract.set_analytics(
      {"encoded": encode64( formData )}
    ).then(() => {
      contract.get_analytics().then(setAnalytics);
      setIsSubmitting(false)
    });
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
    set_analytics: PropTypes.func.isRequired,
    get_analytics: PropTypes.func.isRequired
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
