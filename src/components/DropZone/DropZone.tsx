import React, { ChangeEventHandler } from "react";
import styles from './DropZone.module.sass'

interface IDropZone {
    onChange?: ChangeEventHandler<HTMLInputElement>
    comment?: string
}

const DropZone = ({ onChange, comment }: IDropZone) => {

    const handleClick = () => {
        let inputNode: HTMLInputElement | null = document.querySelector(`input[id=choose-nft-file]`);
        inputNode?.click();
    }

    return (
        <div onClick={ handleClick } className={ styles.wrapper }>
            <label className={ styles.inputLabel } >
                <input id='choose-nft-file' onChange={ onChange } type='file' />
                {'Choose file'}
            </label>
            <div className={ styles.commentSection }>
                { comment }
            </div>
        </div>
    )

}

export default DropZone;
