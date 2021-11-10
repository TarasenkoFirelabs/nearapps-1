import React from "react";
import styles from './SubmitButton.module.sass';

const SubmitButton = ({ text }) => {
    return (
        <button className={ styles.submitButton }>{ text }</button>
    )
}

export default SubmitButton;
