import React, { useCallback, useEffect, useState } from "react";
import styles from './CreateNft.module.sass';
import ChooseFile from "../../components/modules/create-nft/ChooseFile"
import PreviewNFT from "../../components/modules/create-nft/PreviewNFT";
import { postApi } from "../../helpers/api";
import { INftData } from "../../components/modules/create-nft/CreateNFTTypes";


const CreateNft = () => {
    const [step, setStep] = useState(1);
    const [isValid, setIsValid] = useState<boolean>(false)
    const [nftData, setNftData] = useState<INftData>({
        file: null,
        title: '',
        description: '',
        height: '',
        width: ''
    });

    const validate = useCallback(() => {
        let isValid = true;

        if (!nftData.title || !nftData.file || !nftData.description) {
            isValid = false
        }

        setIsValid(isValid);
    },[nftData])

    useEffect(() => {
        validate()
    }, [validate])

    const handleChooseFile = (e: React.ChangeEvent<HTMLInputElement>) => {
        e.preventDefault();
        const { files } = e.target;

        setNftData(prev => {
            if (files) {
                prev.file = URL.createObjectURL(files[0]);
            }
            return { ...prev }
        })
    }

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { value, name } = e.target;

        setNftData(prev => {
            prev[name] = value
            return { ...prev };
        })
    }

    const handleStepNext = () => {
        setStep(step + 1)
    }

    const handleStepBack = () => {
        setStep(step - 1)
    }

    const submitNft = () => {
        postApi('http://example.com/api/endpoint/', {
            nftData
        }, {
            mode: "no-cors",
        }).then(res => {
            console.log(res);
        })
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
                        isValid={ isValid }
                    />)
            case 2:
                return (
                    <PreviewNFT
                        step={ step }
                        handleStepBack={ handleStepBack }
                        submitNft={ submitNft }
                        nftData={ nftData }
                    />)
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
