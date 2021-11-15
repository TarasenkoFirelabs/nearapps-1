import { INftData } from "../CreateNFTTypes";

export interface IChooseFileProps {
    step: number
    handleStepNext: any
    handleChooseFile: any
    handleChange: any
    nftData: INftData
    isValid: boolean
}
