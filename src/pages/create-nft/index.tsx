import React, { useState } from "react";
import styles from './CreateNft.module.sass';
import CancelIcon from "../../assets/icons/CancelIcon";
import Fab from "../../components/Fab";

const CreateNft = () => {
    const [step, setStep] = useState(1);

    return (
        <div className={ styles.root }>
            <div className={ styles.baseBody }>
                <div className={ styles.headerBlock }>
                    <div className='flexBasis25'/>
                    <div className='flexBasis50 textCenter'>
                        {'Create an NFT'}
                    </div>
                    <div className={ styles.cancelFab }>
                        <Fab component={ <CancelIcon/> } />
                    </div>
                </div>
                <hr className={ styles.horizontalLine1 } />
                <div className='flexInlineCenter textSubBody-12 padding-10-0-0'>
                    {`STEP ${ step }  OF 2`}
                </div>

                <input type='file'/>


            </div>
        </div>
    )
}

export default CreateNft;
