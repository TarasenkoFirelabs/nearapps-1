import React, { ChangeEventHandler } from "react";
import styles from './FadeTextArea.module.sass';

interface IFadeTextAreaProps {
    onChange?: ChangeEventHandler<HTMLTextAreaElement>
    label?: string
    rows?: number
}

const FadeTextArea = ({ onChange, label, rows }: IFadeTextAreaProps) => {

    return (
        <div className={ styles.root }>
            <label htmlFor='fade-text-area-1-1' className='textSubBody-12 padding-5-0'>
                { label }
            </label>
            <textarea
                id='fade-text-area-1'
                className={ styles.fadeTextArea }
                onChange={ onChange }
                rows={ rows }
            />
        </div>
    )
}

export default FadeTextArea;
