import React, { useState } from "react";
import styles from './CreateNft.module.sass';
import ChooseFile from "../../components/modules/create-nft/ChooseFile"
import PreviewNFT from "../../components/modules/create-nft/PreviewNFT";

const CreateNft = () => {
    const [step, setStep] = useState(1);

    const handleStepNext = () => {
        setStep(step + 1)
    }

    const handleStepBack = () => {
        setStep(step - 1)
    }

    const getStepPage = () => {
        switch (step) {
            case 1:
                return <ChooseFile step={ step } handleStepNext={ handleStepNext } />
            case 2:
                return <PreviewNFT step={ step } handleStepBack={ handleStepBack } />
            default:
                return;
        }
    }

    return (
        <div className={ styles.root }>
            <div className={ styles.baseBody }>
                { getStepPage() }
            </div>
        </div>
    )
}

export default CreateNft;
