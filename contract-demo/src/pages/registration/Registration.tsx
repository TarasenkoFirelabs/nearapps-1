import React, { useCallback, useEffect, useState } from "react";
import { useAlert } from "react-alert";
import styles from './Registration.module.sass';
import OutlinedInput from "../../components/OutlinedInput";
import Button from "../../components/Button";
import { Colors } from "../../utils/Colors";

const regExpression = /^[a-zA-Z0-9!#$%^&*()_+\-=\[\]{};':"\\|<>\/?]*$/

const Registration = ({ account, nearConfig, currentUser }) => {
    const alert = useAlert();
    const [accountID, setAccountID] = useState<string>('');
    const [isValid, setIsValid] = useState<boolean>(false)

    const validate = useCallback(() => {
        let isValid = true;
        let checkAccountIDReg = regExpression.test(accountID)

        if (!checkAccountIDReg || accountID.length < 2) {
            isValid = false
        }

        setIsValid(isValid);
    },[accountID])

    useEffect(() => {
        validate()
    }, [validate])

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { value } = e.target;
        setAccountID(value);
    }

    const encode64_AccountID = (AccountID) => {
      return encode_utf8_base64(AccountID);
    }

    const submitAccountID = async () => {
      try {
        const args = {
          "tags": {"test": "test", "bla": "bla"},
          "contract_name": accountID,
          "args": {"make_wallet", ""}
        };
        setIsValid(false)

        callContract(account, nearConfig, 'app', args)
          .then((txid) => {
            const link = `https://explorer.${nearConfig.networkId}.near.org/transactions/${txid}`;
            alert.show(link);

            // currentUser.accountId = accountID;
            // account = await near.account(currentUser.accountId);

          })
          .finally(() => {setIsValid(true)})

      } catch(error) {
        alert.error(error);
      }
    }

    return (
        <div className={ styles.root }>
            <div className={ styles.baseBody }>
                <div className={ styles.registrationUnderBody2 }>
                  {currentUser.accountId}
                </div>
                <hr className={ styles.horizontalLine2 } />

                <div className='flexInlineCenter textHeader padding-10-10-0'>
                    {'Create NEAR account'}
                </div>
                <hr className={ styles.horizontalLine1 } />
                <div className={ styles.registrationSubBody0 }>
                    {'Enter an Account ID to use with your NEAR account. Your Account ID will be used for all NEAR operations, ' +
                    'including sending and receiving assets.\n'}
                </div>
                <div className='flexInlineCenter'>
                     <OutlinedInput onChange={ handleChange } label={'ACCOUNT ID'} />
                </div>
                <div className={ styles.registrationSubBody1 }>
                    {'Your account ID can contain any of the following:\n'}
                </div>

                <ul className={ styles.registrationListBody1 }>
                    <li>{ 'Lowercase characters (a-z)'}</li>
                    <li>{'Digits (0-9)'}</li>
                    <li>{'Characters (_-) can be used as separators'}</li>
                </ul>
                <div className={ styles.registrationSubBody3 }>
                    {'Your account ID CANNOT contain:'}
                </div>
                <ul className={ styles.registrationListBody2 }>
                    <li>{'Characters "@" or "."'}</li>
                    <li>{'Fewer than 2 characters'}</li>
                    <li>{'More than 64 characters (including .near)'}</li>
                </ul>
                <div className='flexInlineCenter'>
                    <Button
                        onClick={ submitAccountID }
                        text='Create'
                        textColor={ Colors.white }
                        backgroundColor={ Colors.blue }
                        disabled={ !isValid }
                    />
                </div>
                <div className={ styles.registrationUnderBody1 }>
                    {'By creating a NEAR account, you agree to the NEAR Wallet'} &#160;
                    <a
                      href="https://wallet.testnet.near.org/20210319-near-tcs.a776bb61.pdf"
                      className={ styles.registrationLink }>
                      {'Terms of Service'}
                    </a>
                  &#160;{'and'} &#160;
                    <a
                      href="https://near.org/privacy/"
                      className={ styles.registrationLink }>
                      {'Privacy Policy.'}
                    </a>
                </div>
            </div>
        </div>
    )
}

export default Registration;
