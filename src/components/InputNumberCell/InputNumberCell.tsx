import React from "react";
import styles from './InputNumberCell.module.sass';

const InputNumberCell = ({ onChange, name }) => {

    return (
        <div>
            <input
                className={ styles.numberField }
                onChange={ onChange }
                name={ name }
                type='text'
                maxLength={ 1 }
            />
        </div>
    )

}

export default InputNumberCell;
