import React from "react";
import styles from './OutlinedInput.module.sass';

const OutlinedInput = ({ onChange = null, label }) => {

    return (
        <div className='flexColumnCenter'>
            <label htmlFor='outlined-input-1' className='textSubBody-12 padding-5-0'>
                { label }
            </label>
            <input
                id='outlined-input-1'
                className={ styles.outlinedInput }
                onChange={ onChange }
                maxLength={ 64 }
            />
        </div>
    )
}

export default OutlinedInput;
