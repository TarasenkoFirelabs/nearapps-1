import React, { MouseEventHandler, ReactComponentElement } from "react";
import styles from './Fab.module.sass';

interface IFabProps {
    onClick?: MouseEventHandler
    component?: ReactComponentElement<any>,
    backgroundColor?: string,
    color?: string,
    disabled?: boolean
}

const Fab = ({ onClick, component, backgroundColor = 'transparent', color, disabled = false }: IFabProps) => {

    return (
       <button
           onClick={ onClick }
           className={ styles.fabButton }
           style={{ backgroundColor: backgroundColor }}
           color={ color }
           disabled={ disabled }
       >
           { component }
       </button>
    )
}

export default Fab
