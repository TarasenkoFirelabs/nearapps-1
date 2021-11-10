import React, {useCallback, useEffect, useState} from "react";
import styles from './Registration.module.sass';
import OutlinedInput from "../../../components/OutlinedInput";
import Button from "../../../components/Button";
import { Colors } from "../../../utils";

const regExpression = /^[a-zA-Z0-9!#$%^&*()_+\-=\[\]{};':"\\|<>\/?]*$/

const Registration = () => {
    const [accountID, setAccountID] = useState('');
    const [isValid, setIsValid] = useState(false)

    const validate = useCallback(() => {
        let isValid = true;
        let checkAccountIDReg = regExpression.test(accountID)

        if (!checkAccountIDReg|| !accountID) {
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

    return (
        <div className={ styles.root }>
            <div className={ styles.baseBody }>
                <div className='flexInlineCenter textHeader padding-10-10-0'>
                    {'Create NEAR account'}
                </div>
                <hr className={ styles.horizontalLine1 } />
                <div className='flexInlineCenter textSubBody-12 padding-10-0-0'>
                    {'STEP 1 OF 3'}
                </div>
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
                        text='Create'
                        textColor={ Colors.white }
                        backgroundColor={ Colors.blue }
                        disabled={ !isValid }
                    />
                </div>
                <div className={ styles.registrationUnderBody1 }>
                    {'By creating a NEAR account, you agree to the NEAR Wallet'} &#160;
                    <a href='#' className={ styles.registrationLink }>{'Terms of Service'}</a> &#160;
                    {'and'} &#160;
                    <a href='#' className={ styles.registrationLink }>{'Privacy Policy.'}</a>
                </div>
                <hr className={ styles.horizontalLine2 } />
                <div className={ styles.registrationUnderBody2 }>
                    {'Already have an account?'} &#160;
                    <a href='#' className={ styles.registrationLink }>{'Sign in'}</a>
                </div>
            </div>
        </div>
    )
}

export default Registration;
