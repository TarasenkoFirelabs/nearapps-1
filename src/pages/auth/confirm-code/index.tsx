import Image from "next/image";
import React, {useCallback, useEffect, useState} from "react";
import { Icons, Colors } from '../../../utils';
import styles from './ConfirmCode.module.sass';
import InputNumberCell from "../../../components/InputNumberCell";
import Button from "../../../components/Button";
import { postApi } from "../../../helpers/api";

const ConfirmCode = () => {
    // @ts-ignore
    const [isPhoneNumber, setIsPhoneNumber] = useState<boolean>(false);
    const [code, setCode] = useState<any[]>(new Array(6).fill(''));
    const [isValid, setIsValid] = useState<boolean>(false)

    const validate = useCallback(() => {
        let isValid = true;

        if (code.join('').length < 6) {
            isValid = false
        }

        setIsValid(isValid);
    },[code])

    useEffect(() => {
        validate()
    }, [validate])

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>, key: number) => {
        const { value } = e.target;

        setCode((prev: any[]) => {
            let arr = prev;
            arr[key] = value;
            return [...arr];
        });

    }

    const handleFocus = (e: React.KeyboardEvent<HTMLInputElement>, key: number) => {
        let index: number = e.key !== 'Backspace'
            ? key + 1
            : key - 1

        const nextField: HTMLInputElement | null = document.querySelector(
            `input[name=verifyCell-${index}]`
        );

        if (nextField) {
            nextField.focus();
        }
    }

    const submitCode = () => {
        postApi('http://example.com/api/endpoint/', {
            code: code.join('')
        }, {
            mode: "no-cors",
        }).then(res => {
            console.log(res)
        })
    }

    const getInputCells = () => {
        return code.map( (_item, key) => {
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
        <div className={ styles.root }>
            <div className={ styles.baseBody }>
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
                    <Button
                        onClick={ submitCode }
                        text={'Verify & Continue'}
                        backgroundColor={ Colors.blue }
                        textColor={ Colors.white }
                        disabled={ !isValid }
                    />
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
                    <Button text={'Resend your code'} textColor={ Colors.primary } />
                </div>
            </div>
        </div>
    )
}

export default ConfirmCode;
