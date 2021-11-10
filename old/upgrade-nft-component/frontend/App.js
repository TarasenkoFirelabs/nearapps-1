import 'regenerator-runtime/runtime';
import React, { useState, useEffect } from 'react';
import { useAlert } from 'react-alert'
import PropTypes from 'prop-types';
import Form from './components/Form';
import SignIn from './components/SignIn';
import { callContract, getNearExplorerTxLink } from './utils/CallContract';
import { TagChart } from './tagChart/TagChart';

const App = ({ account, contract, currentUser, nearConfig, wallet }) => {
  let initData = { accountId: currentUser.accountId };

  const [tags, setTags] = useState(initData);
  const [isSubmitting, setIsSubmitting] = useState(false)

  const handleSubmit = (value) => {
    setTags([...tags, value]);
  }

  function encode_utf8_base64(str) {
    return Buffer.from(str).toString('base64');
  }

  const encode64 = (formData) => {
    return encode_utf8_base64(formData.app_id) + '_' +
      encode_utf8_base64(formData.action_id) + '_' +
      encode_utf8_base64(formData.user_name);
  }

  const alert = useAlert();

  const onSubmit = (formData) => {
    setIsSubmitting(true);

    try {
      const time = Math.floor(Date.now() / 1000);
      const args = {"accountId": "", "time": time, "tags": encode64( formData )};

      callContract(account, nearConfig, 'call_near_apps', args)
        .then((txid) => {
          setIsSubmitting(false)

          alert.show(getNearExplorerTxLink(nearConfig, txid));
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
      <TagChart callback={handleSubmit}/>
      { currentUser
        ? <Form onSubmit={onSubmit} currentUser={currentUser} tags={tags} disabled={isSubmitting} />
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
  tags: PropTypes.shape({
    tags: PropTypes.string.isRequired
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
