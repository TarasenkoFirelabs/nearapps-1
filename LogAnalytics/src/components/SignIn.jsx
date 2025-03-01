import React from 'react';

export default function SignIn() {
  return (
    <>
      <p>
          This app demonstrates a key element of NEAR’s UX: once an app has
          permission to make calls on behalf of a user (that is, once a user
          signs in), the app can make calls to the blockhain for them without
          prompting extra confirmation.
      </p>
      <p>
          Go ahead and sign in to try it out!
      </p>
    </>
  );
}
