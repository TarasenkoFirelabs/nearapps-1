import { Button } from "antd";
import Image from "next/image";
import React, { useState } from "react";
import { Colors, Icons } from "../../../utils";
import { AuthStore } from "../../../stores/AuthStore";
import { observer } from "mobx-react";
import { toJS } from "mobx";

const SignUp = observer(() => {
  const { wallet, signIn } = AuthStore;
  const [inputType, setInputType] = useState("email");
  const walletData = toJS(wallet);
 
  const handleInputType = (type) => {
    setInputType(type);
  };

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
          <div className="SignUp__logoContainer-row-logoText">
            <p>labs</p>
          </div>
        </div>
      </div>
      <div className="SignUp__formContainer">
        <div className="SignUp__formContainer-form">
          <p style={{ opacity: 0.8 }}>Continue with</p>
          <div className="SignUp__formContainer-form-tab">
            <Button
              className="SignUp__formContainer-form-tab-button"
              style={{
                backgroundColor: inputType === "email" && Colors.primary,
              }}
              onClick={() => handleInputType("email")}
            >
              Email
            </Button>
            <Button
              className="SignUp__formContainer-form-tab-button"
              style={{
                backgroundColor: inputType === "phone" && Colors.primary,
              }}
              onClick={() => handleInputType("phone")}
            >
              Phone
            </Button>
          </div>
          <div className="SignUp__formContainer-form-inputContainer">
            {inputType === "email" ? (
              <input type="email" placeholder="Email Address" required />
            ) : (
              <input type="phone" placeholder="Ex. +1488494433" required />
            )}
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
          <p className="SignUp__formContainer-form-term">
            by clicking continue you must agree to near labs
            <br />
            <a
              style={{ color: "lightblue" }}
              href="https://wallet.testnet.near.org/20210319-near-tcs.a776bb61.pdf"
              target="_blank"
            >
              {" "}
              Terms &amp; Conditions
            </a>{" "}
            and
            <a
              style={{ color: "lightblue" }}
              href="https://near.org/privacy/"
              target="_blank"
            >
              {" "}
              Privacy Policy
            </a>
          </p>
          <hr className="SignUp__formContainer-line" />
          <p className="SignUp__formContainer-textButton">
            Already have Near Account?
          </p>
          <Button
            className="SignUp__formContainer-loginButton"
            onClick={() => {
              const successRedirectUrl = (process.env.REACT_APP_HOST || "http://localhost:3000/") + 'auth/registration';
              const failedRedirectUrl = (process.env.REACT_APP_HOST || "http://localhost:3000/") + 'auth/sign-up';
              signIn(successRedirectUrl, failedRedirectUrl)
            }}
          >
            Log in with NEAR
          </Button>
        </div>
      </div>
    </div>
  );
});

export default SignUp;
