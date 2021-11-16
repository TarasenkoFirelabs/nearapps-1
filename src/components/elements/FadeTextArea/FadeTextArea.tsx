import React, { ChangeEventHandler } from "react";
import styles from './FadeTextArea.module.sass';

interface IFadeTextAreaProps {
    onChange?: ChangeEventHandler<HTMLTextAreaElement>
    name?: string
    value?: any
    label?: string
    rows?: number
    maxLength?: number
}

const FadeTextArea = ({ onChange, name, value, label, rows, maxLength }: IFadeTextAreaProps) => {

    return (
        <div className={ styles.root }>
            <label htmlFor='fade-text-area-1-1' className='textSubBody-12 padding-5-0'>
                { label }
            </label>
            <textarea
                id='fade-text-area-1'
                className={ styles.fadeTextArea }
                name={ name }
                value={ value }
                onChange={ onChange }
                rows={ rows }
                maxLength={ maxLength }
            />
        </div>
    )
}

export default FadeTextArea;
