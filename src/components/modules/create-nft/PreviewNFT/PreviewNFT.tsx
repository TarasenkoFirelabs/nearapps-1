import React from "react";
import styles from "./PreviewNFT.module.sass";
import Fab from "../../../elements/Fab";
import CancelIcon from "../../../../assets/icons/CancelIcon";
import BackIcon from "../../../../assets/icons/BackIcon";
import Image from "next/image";
import Button from "../../../Button";
import { Colors } from "../../../../utils";
import { IPreviewNFTProps } from "./PreviewNFTTypes";
import profile from '../../../../assets/images/profile_no_avatar.svg';

const PreviewNFT = ({ step, handleStepBack, nftData, submitNft }: IPreviewNFTProps) => {

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
            <div className={ styles.imagePreviewBlock }>
                    <div className={ styles.blockPreview }>{'Preview'}</div>
                    <Image
                        src={ nftData?.file }
                        width={ 313 }
                        height={ 186 }
                        objectFit='contain'
                        objectPosition='50% 50%'
                        alt='NFT preview image'
                    />
                <div className={ styles.subHeaderBlock }>
                    { nftData?.title }
                </div>
                <div className={ styles.chipsBlock }>
                    {'Digital art'}
                </div>
                <div className={ styles.profileInfoBlock }>
                    <div>
                        <Image
                            src={ profile }
                            alt='profile image'
                            width={ 40 }
                            height={ 40 }
                        />
                    </div>
                    <div className={ styles.profileContent }>
                        <span className='textSubHeader-13'>{'john_doe'}</span>
                        <span className='textSubBody-12'>{'0xkej34i04...3cdk'}</span>
                    </div>
                </div>
                <div className={ styles.descriptionBlock }>
                    <div className='textHeader-16'>
                        {'Descriptions'}
                    </div>
                    <div className='textSubBody-13 padding-5-0-0'>
                        { nftData?.description }
                    </div>
                </div>
            </div>
            <div className='flexInlineCenter padding-20-0-0'>
                <Button
                    className={ styles.mintBtn }
                    onClick={ submitNft }
                    text='Mint NFT'
                    backgroundColor={ Colors.primary }
                    textColor={ Colors.white }
                />
            </div>
        </>
    )
}

export default PreviewNFT;
