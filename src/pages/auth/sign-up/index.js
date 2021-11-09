import { Button } from "antd";
import { Colors, Icons } from '../../../utils';
import Image from "next/image";
import React, { useState, useContext, useEffect } from 'react';
// import { initNear, hasKey } from "../../../state/near_old";
// import { appStore, onAppMount } from '../../../state/app';

function SignUp() {

  
  //initNear();

  // const { state, dispatch, update } = useContext(appStore);

  // const onMount = () => {
  //   dispatch(onAppMount());
  // };
  // useEffect(onMount, []);

  // console.log('state', state);

  // const { wallet } = state;

  const [inputType, setInputType] = useState('email');

  const handleInputType = type => {
    setInputType(type);
  }

  return (
    <div className="SignUp">
      <div className="SignUp__logoContainer">
        <div className="SignUp__logoContainer-row">
          <Image
            className="SignUp__logoContainer-row-logo"
            alt=""
            src={Icons.logo}
            width="100px"
            height="100%"
          />
          <div className="SignUp__logoContainer-row-logoText"><p>labs</p></div>
        </div>
      </div>
      <div className="SignUp__formContainer">
        <div className="SignUp__formContainer-form">
          <p style={{opacity: 0.8}}>Continue with</p>
          <div className="SignUp__formContainer-form-tab">
            <Button
              className="SignUp__formContainer-form-tab-button"
              style={{ backgroundColor: inputType === 'email' && Colors.primary }}
              onClick={() => handleInputType('email')}
            >Email</Button>
            <Button
              className="SignUp__formContainer-form-tab-button"
              style={{ backgroundColor: inputType === 'phone' && Colors.primary }}
              onClick={() => handleInputType('phone')}
            >Phone</Button>
          </div>
          <div className="SignUp__formContainer-form-inputContainer">
            {inputType === 'email' ?
              <input type="email" placeholder="Email Addres" required /> :
              <input type="phone" placeholder="Ex. +1488494433" required />}
            <Button className="SignUp__formContainer-form-inputContainer-arrowRight">
              <Image
                className="SignUp__logoContainer-row-logo"
                alt=""
                src={Icons.arrowRight}
                width="16px"
                height="16px"
              />
            </Button>
          </div>
          <p className="SignUp__formContainer-form-term">by clicking continue you must agree to near labs
            <a style={{ color: 'lightblue' }}> Terms & Conditions</a> ans
            <a style={{ color: 'lightblue' }}>  Privacy Policy</a>
          </p>
          <hr className='SignUp__formContainer-line' />
          <p className='SignUp__formContainer-textButton'>Already have Near Account?</p>
          <Button className="SignUp__formContainer-loginButton" onClick={() => wallet.signIn()}>Log in with NEAR</Button>
        </div>
      </div>
    </div >
  )
}

export default SignUp
