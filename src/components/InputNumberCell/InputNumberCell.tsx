import React, { ChangeEventHandler, KeyboardEventHandler } from "react";
import styles from './InputNumberCell.module.sass';

interface IInputNumberCellProps {
    onChange?: ChangeEventHandler<HTMLInputElement>
    handleFocus?: KeyboardEventHandler<HTMLInputElement>
    name: string
}

const InputNumberCell = ({ onChange, handleFocus, name }: IInputNumberCellProps) => {

    return (
        <div>
            <input
                className={ styles.numberField }
                onChange={ onChange }
                onKeyUp={ handleFocus }
                name={ name }
                type='text'
                maxLength={ 1 }
            />
        </div>
    )

}

export default InputNumberCell;
