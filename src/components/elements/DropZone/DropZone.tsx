import React, { ChangeEventHandler, RefObject, useRef } from "react";
import styles from './DropZone.module.sass'

interface IDropZone {
    onChange?: ChangeEventHandler<HTMLInputElement>
    name?: string
    labelName?: string
    comment?: string
    acceptType?: string
}

const DropZone = ({ onChange, name, labelName, comment, acceptType }: IDropZone) => {
    const ref: RefObject<any> = useRef();

    const handleClick = (e) => {
        if (e.target !== e.currentTarget){
            return;
        }
        ref.current.click();
    }

    return (
        <div onClick={ handleClick } className={ styles.wrapper }>
            <label className={ styles.inputLabel } >
                <input
                    ref={ ref }
                    name={ name }
                    onChange={ onChange }
                    type='file'
                    accept={ acceptType }
                />
                { labelName }
            </label>
            <div className={ styles.commentSection }>
                { comment }
            </div>
        </div>
    )

}

export default DropZone;
