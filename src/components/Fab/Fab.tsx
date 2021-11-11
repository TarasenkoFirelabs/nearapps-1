import React, { MouseEventHandler, ReactComponentElement } from "react";
import styles from './Fab.module.sass';

interface IFabProps {
    onClick?: MouseEventHandler
    component?: ReactComponentElement<any>,
    backgroundColor?: string,
    disabled?: boolean
}

const Fab = ({ onClick, component, backgroundColor = 'transparent', disabled = false }: IFabProps) => {

    return (
       <button
           onClick={ onClick }
           className={ styles.fabButton }
           style={{ backgroundColor: backgroundColor }}
           disabled={ disabled }
       >
           { component }
       </button>
    )
}

export default Fab
