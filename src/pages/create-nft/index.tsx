import React, {useState} from "react";
import styles from './CreateNft.module.sass';
import ChooseFile from "../../components/modules/create-nft/ChooseFile"
import PreviewNFT from "../../components/modules/create-nft/PreviewNFT";

interface INftData {
    path: string
    file: any
    title: string
    description: string
    height: string
    width: string
}

const CreateNft = () => {
    const [step, setStep] = useState(1);
    const [nftData, setNftData] = useState<INftData>({
        path: '',
        file: null,
        title: '',
        description: '',
        height: '',
        width: ''
    });

    const handleChooseFile = (e: React.ChangeEvent<HTMLInputElement>) => {
        e.preventDefault();
        const { files } = e.target;

        setNftData(prev => {
            if (files) {
                prev.file = URL.createObjectURL(files[0]);
            }
            return { ...prev }
        })


        /*for(let i = 0; i < filesLength; i++) {
            let reader = new FileReader();
            let file = files[i];

            reader.onloadend = () => {
                setNftData(prev => {

                    {
                        images: self.state.images.concat(reader.result);
                    }
                }});
            }

            reader.readAsDataURL(file);
        }*/


       /* setNftData(prev => {
            prev.path = value
            return { ...prev };
        })*/
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
                return <PreviewNFT step={ step } handleStepBack={ handleStepBack } nftData={ nftData } />
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
