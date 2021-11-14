import styles from './ChooseFile.module.sass';
import React from "react";
import Fab from "../../../elements/Fab";
import CancelIcon from "../../../../assets/icons/CancelIcon";
import DropZone from "../../../elements/DropZone";
import OutlinedInput from "../../../OutlinedInput";
import { Colors } from "../../../../utils";
import FadeTextArea from "../../../elements/FadeTextArea";
import Button from "../../../Button";

const ChooseFile = ({ step, handleStepNext }) => {

    const handleChooseFile = (e) => {

        console.log(e)

    }

    return (
        <>
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
                {`STEP ${ step } OF 2`}
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
            <div className='flexInlineCenter padding-20-0-0'>
                <FadeTextArea
                    label={'DESCRIPTION'}
                    rows={ 3 }
                />
            </div>
            <div className={ styles.propertiesBlock }>
                <div className='width-335'>
                    {'PROPERTIES'}
                </div>
                <div className='flexInlineCenter'>
                    <OutlinedInput className={ styles.outInput } />
                    <OutlinedInput className={ styles.outInput } />
                </div>
            </div>
            <div className='spacer-15' />
            <hr className={ styles.horizontalLine1 } />
            <div className={ styles.nextButtonBlock }>
                <div className='width-335'>
                    <Button
                        className={ styles.nextButton }
                        onClick={ handleStepNext }
                        text='Next'
                        backgroundColor={ Colors.black }
                        textColor={ Colors.white }
                    />
                </div>
            </div>
            <div className='spacer-15' />
        </>
    )
}

export default ChooseFile;
