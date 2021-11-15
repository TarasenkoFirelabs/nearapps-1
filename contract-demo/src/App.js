import 'regenerator-runtime/runtime';
import React, { useState, useEffect } from 'react';
import PropTypes from 'prop-types';
import RegistrationForm from './pages/registration/Registration';
import SignIn from './pages/sign-in/SignIn';

const App = ({ contract, currentUser, nearConfig, wallet }) => {
  useEffect(() => {
    // 
  }, []);

  const signIn = () => {
    wallet.requestSignIn(
      nearConfig.contractName,
      'NEAR Apps'
    );
  };

  const signOut = () => {
    wallet.signOut();
    window.location.replace(window.location.origin + window.location.pathname);
  };

  return (
    <main>
      <header>
        <h1>NEAR Apps</h1>
        { currentUser
          ? <button onClick={signOut}>Log out</button>
          : <button onClick={signIn}>Log in</button>
        }
      </header>
      { currentUser
        ? <RegistrationForm contract={contract} currentUser={currentUser} nearConfig={nearConfig} />
        : <SignIn/>
      }
    </main>
  );
};

App.propTypes = {
  contract: PropTypes.shape({
    call: PropTypes.func.isRequired,
    log_analytics: PropTypes.func.isRequired
  }).isRequired,
  currentUser: PropTypes.shape({
    accountId: PropTypes.string.isRequired,
    balance: PropTypes.number.isRequired
  }),
  // analytics: PropTypes.shape({
  //   app_id: PropTypes.string.isRequired,
  //   action_id: PropTypes.string.isRequired,
  //   user_name: PropTypes.string.isRequired
  // }),
  nearConfig: PropTypes.shape({
    contractName: PropTypes.string.isRequired
  }).isRequired,
  wallet: PropTypes.shape({
    requestSignIn: PropTypes.func.isRequired,
    signOut: PropTypes.func.isRequired
  }).isRequired
};

export default App;
