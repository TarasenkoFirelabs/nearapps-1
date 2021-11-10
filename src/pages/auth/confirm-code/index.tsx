import Image from "next/image";
import React, { useState } from "react";
import { Icons, Colors } from '../../../utils';
import styles from "../../../../styles/ConfirmCode.module.sass";
import InputNumberCell from "../../../components/InputNumberCell";
import SubmitButton from "../../../components/SubmitButton";
import Button from "../../../components/Button";

const ConfirmCode = () => {
    // @ts-ignore
    const [isPhoneNumber, setIsPhoneNumber] = useState<boolean>(false);
    const [code, setCode] = useState<any[]>(new Array(6).fill(''));

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>, key: number) => {
        const { value } = e.target;

        setCode((prev: any[]) => {
            let arr = prev;
            arr[key] = value;
            return [...arr];
        });

    }

    const handleFocus = (e, key: number) => {

        let index: number = e.key !== 'Backspace'
            ? key + 1
            : key - 1

        // @ts-ignore
        const nextField: HTMLInputElement = document.querySelector(
            `input[name=verifyCell-${index}]`
        );

        if (nextField) {
            nextField.focus();
        }
    }

    const getInputCells = () => {
        return code.map( (item, key) => {
            return (
                <form className='padding-5' key={ key }>
                    <InputNumberCell
                        onChange={ (e) => handleChange(e, key) }
                        handleFocus={ (e) => handleFocus(e, key) }
                        name={`verifyCell-${ key }`}
                    />
                </form>
            )
        })
    }

    return (
        <div className='textCenter'>
            <div className='flexInlineCenter'>
                <Image className="SignUp__logoContainer-row-logo" src={ Icons.nearLogoBlack } alt="Near Logo" width="100px" height="50%" />
                <div className="logoContainerLabs ml-3">{'labs'}</div>
            </div>
            <hr className={ styles.horizontalLine1 }/>
            <p className={ styles.verificationTitle }>
                {'Enter Verification Code'}
            </p>
            <div className={ styles.verificationBody }>
                {'We\'ve sent a 6-digit verification code to'} &#160;
                { isPhoneNumber ? 'phone number' : 'the email address'}
            </div>
            <div className={ styles.verificationCredentials }>{ isPhoneNumber ? '+380634567543' : 'test@gmail.com' }</div>
            <div className={ styles.verificationSubBody }>
                {'Enter verification code'}
            </div>

            <div className='flexInlineCenter'>
                { getInputCells() }
            </div>
            <div className='flexInlineCenter padding-30-0-0'>
                <SubmitButton text={'Verify & Continue'}/>
            </div>
            <div className='flexInlineCenter padding-10-0-0'>
                <Button text={'Cancel'}/>
            </div>
            <hr className={ styles.horizontalLine2 } />
            <div className='textSubBody-14 padding-10'>
                <a href='#'>{'Didn\'t receive your code?'}</a>
            </div>
            <div className={ styles.textLink }>
                <a href='#'>{'Send to different email address'}</a>
            </div>
            <div className={ styles.textButton }>
                <Button text={'Resend your code'} textColor={ Colors.primary }/>
            </div>
        </div>
    )
}

export default ConfirmCode;
