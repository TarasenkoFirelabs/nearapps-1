import React, {useState} from "react";
import styles from './CreateNft.module.sass';
import ChooseFile from "../../components/modules/create-nft/ChooseFile"
import PreviewNFT from "../../components/modules/create-nft/PreviewNFT";

const CreateNft = () => {
    const [step, setStep] = useState(1);
    const [nftData, setNftData] = useState({
        path: '',
        title: '',
        description: '',
        height: '',
        width: ''
    });

    const handleChooseFile = (e: React.ChangeEvent<HTMLInputElement>) => {
        const {value} = e.target;

        setNftData(prev => {
            prev.path = value
            return { ...prev };
        })
    }

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { value, name } = e.target;

        setNftData(prev => {
            prev[name] = value
            return { ...prev };
        })
    }
    console.log(nftData)

    const handleStepNext = () => {
        setStep(step + 1)
    }

    const handleStepBack = () => {
        setStep(step - 1)
    }

    const getStepPage = () => {
        switch (step) {
            case 1:
                return (
                    <ChooseFile
                        step={ step }
                        handleStepNext={ handleStepNext }
                        handleChooseFile={ handleChooseFile }
                        handleChange={ handleChange }
                        nftData={ nftData }
                    />)
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
