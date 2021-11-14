import React from "react";
import styles from "./PreviewNFT.module.sass";
import Fab from "../../../elements/Fab";
import CancelIcon from "../../../../assets/icons/CancelIcon";
import BackIcon from "../../../../assets/icons/BackIcon";

const PreviewNFT = ({ step, handleStepBack }) => {

    return (
        <>
            <div className={ styles.headerBlock }>
                <div className={ styles.backFab }>
                    <Fab onClick={ handleStepBack } component={ <BackIcon /> } />
                </div>
                <div className='flexBasis50 textCenter'>
                    {'Preview NFT'}
                </div>
                <div className={ styles.cancelFab }>
                    <Fab component={ <CancelIcon/> } />
                </div>
            </div>
            <hr className={ styles.horizontalLine1 } />
            <div className='flexInlineCenter textSubBody-12 padding-10-0-0'>
                {`STEP ${ step } OF 2`}
            </div>
        </>
    )
}

export default PreviewNFT;
