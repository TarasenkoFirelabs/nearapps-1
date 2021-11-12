import React, {ChangeEventHandler} from "react";
import styles from './OutlinedInput.module.sass';

interface IOutlinedInputProps {
    onChange?: ChangeEventHandler<HTMLInputElement>
    label?: string
    borderColor?: string
}

const OutlinedInput = ({ onChange, label, borderColor }: IOutlinedInputProps) => {

    return (
        <div className={ styles.root }>
            <label htmlFor='outlined-input-1' className='textSubBody-12 padding-5-0'>
                { label }
            </label>
            <input
                id='outlined-input-1'
                type='text'
                style={{ borderColor: borderColor }}
                className={ styles.outlinedInput }
                onChange={ onChange }
                maxLength={ 64 }
            />
        </div>
    )
}

export default OutlinedInput;
