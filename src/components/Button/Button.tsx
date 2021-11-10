import React from "react";
import { Colors } from "../../utils";
import styles from './Button.module.sass';

const Button = ({ text, backgroundColor = 'transparent', textColor = Colors.grey, disabled = false }) => {
    return (
        <button
            className={ styles.button }
            style={{
                backgroundColor: disabled ? Colors.lightGrey: backgroundColor,
                color: disabled ? Colors.grey : textColor
            }}
            disabled={ disabled }
        >
            { text }
        </button>
    )
}

export default Button;
