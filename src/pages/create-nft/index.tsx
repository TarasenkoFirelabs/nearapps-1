import React, { useState } from "react";
import styles from './CreateNft.module.sass';
import CancelIcon from "../../assets/icons/CancelIcon";
import Fab from "../../components/Fab";
import DropZone from "../../components/DropZone";
import OutlinedInput from "../../components/OutlinedInput";
import {Colors} from "../../utils";
import FadeTextArea from "../../components/FadeTextArea";

const CreateNft = () => {
    // @ts-ignore
    const [step, setStep] = useState(1);

    const handleChooseFile = (e) => {

        console.log(e)

    }

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
                <div className={ styles.inputLabel }>
                    {'Upload Files'}
                </div>
                <div className='flexInlineCenter'>
                    <DropZone
                        onChange={ handleChooseFile }
                        labelName={'Choose file'}
                        comment='PNG, GIF, WEBP, MP4 or MP3. Max 100mb.'
                        acceptType='.PNG,.GIF,.WEBP,.MP4,.MP3'
                    />
                </div>
                <div className='flexInlineCenter padding-30-0-0'>
                    <OutlinedInput label={'TITLE'} borderColor={ Colors.black } />
                </div>
                <div className='flexInlineCenter padding-30-0-0'>
                    <FadeTextArea
                        label={'DESCRIPTION'}
                        rows={ 3 }
                    />
                </div>

            </div>
        </div>
    )
}

export default CreateNft;
