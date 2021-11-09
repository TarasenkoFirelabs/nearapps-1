import React from "react";
import styles from './Button.module.sass';

const Button = ({ text, backgroundColor = 'transparent', textColor = '#808080' }) => {
    return (
        <button
            className={ styles.button }
            style={{
                backgroundColor: backgroundColor,
                color: textColor
            }}
        >
            { text }
        </button>
    )
}

export default Button;
