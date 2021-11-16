import styles from "./GiftNft.module.sass";
import React from "react";
import Fab from "../../components/elements/Fab";
import CancelIcon from "../../assets/icons/CancelIcon";

const GiftNft = () => {

    return (
        <div className={ styles.root }>
            <div className={ styles.baseBody }>
                <div className={ styles.headerBlock }>
                    <div className='flexBasis25'/>
                    <div className='flexBasis50 textCenter'>
                        {'GIFT an NFT'}
                    </div>
                    <div className={ styles.cancelFab }>
                        <Fab component={ <CancelIcon/> } />
                    </div>
                </div>
                <hr className={ styles.horizontalLine1 } />
            </div>
        </div>
    )
}

export default GiftNft;
