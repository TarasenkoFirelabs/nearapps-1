// import * as nearAPI from "near-api-js"
// import { get, set, del } from "../utils/storage"
// import { config } from "./config"

// export const {
//     FUNDING_DATA, FUNDING_DATA_BACKUP, ACCOUNT_LINKS, GAS, SEED_PHRASE_LOCAL_COPY,
//     networkId, nodeUrl, walletUrl, nameSuffix,
//     contractName,
// } = config

// const {
//     KeyPair,
//     InMemorySigner,
//     // transactions: {
//     //     addKey, deleteKey, fullAccessKey
//     // },
//     utils: {
//         PublicKey,
//         format: {
//             parseNearAmount, formatNearAmount
//         }
//     }
// } = nearAPI

// class Near {
//     constructor() {
//         const near = await nearAPI.connect({
//             networkId, nodeUrl, walletUrl, deps: { keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore() },
//         });


//     }



//     const isAccountTaken = async (accountId) => {
//         const account = new nearAPI.Account(near.connection, accountId);
//         try {
//             await account.state()
//         } catch(e) {
//             console.warn(e)
//             if (/does not exist while viewing/.test(e.toString())) {
//                 return false
//             }
//         }
//         return true
//     }
//     // check localLinks, see if they're still valid
//     const localLinks = get(ACCOUNT_LINKS, []).sort((a) => a.claimed ? 1 : -1)
//     for (let i = 0; i < localLinks.length; i++) {
//         const { key, accountId, keyStored = 0, claimed } = localLinks[i]
//         const exists = await isAccountTaken(accountId)
//         if (!exists) {
//             localLinks.splice(i, 1)
//             continue
//         }
//         if (!!claimed || Date.now() - keyStored < 5000) {
//             continue
//         }
//         const keyExists = await hasKey(key, accountId, near)
//         if (!keyExists) {
//             localLinks[i].claimed = true
//         }
//     }
//     set(ACCOUNT_LINKS, localLinks)

//     const claimed = localLinks.filter(({claimed}) => !!claimed)
//     const links = localLinks.filter(({claimed}) => !claimed)

//     // resume wallet / contract flow
//     const wallet = new nearAPI.WalletAccount(near);
//     wallet.signIn = () => {
//         wallet.requestSignIn(contractName, 'Blah Blah')
//     }
//     wallet.signedIn = wallet.isSignedIn()
//     if (wallet.signedIn) {
//         wallet.balance = formatNearAmount((await wallet.account().getAccountBalance()).available, 2)
//     }

//     const contract = await new nearAPI.Contract(wallet.account(), contractName, {
//         changeMethods: ['send', 'create_account', 'create_account_and_claim'],
//     })
//     wallet.isAccountTaken = async (accountId) => {
//         const accountTaken = await isAccountTaken(accountId + nameSuffix)
//         update('app', { accountTaken, wasValidated: true })
//     }
//     wallet.fundAccount = async (amount, accountId, recipientName) => {
//         if (accountId.indexOf(nameSuffix) > -1 || accountId.indexOf('.') > -1) {
//             alert(nameSuffix + ' is added automatically and no "." is allowed. Please remove and try again.')
//             return update('app.wasValidated', true)
//         }
//         accountId = accountId + nameSuffix
//         if (parseFloat(amount, 10) < 0.1 || accountId.length < 2 || accountId.length > 48) {
//             return update('app.wasValidated', true)
//         }
//         const keyPair = KeyPair.fromRandom('ed25519')

//         const links = get(ACCOUNT_LINKS, [])
//         links.push({ key: keyPair.secretKey, accountId, recipientName, keyStored: Date.now() })
//         set(ACCOUNT_LINKS, links)

//         // set(FUNDING_DATA, { key: keyPair.secretKey, accountId, recipientName, amount, funder_account_id: wallet.getAccountId() })
//         await contract.create_account({ new_account_id: accountId, new_public_key: keyPair.publicKey.toString() }, GAS, parseNearAmount(amount))
//     }

//     update('', { near, wallet, links, claimed })
// };



import { observable, action } from 'mobx';
import { createBrowserHistory } from 'history';
import * as nearAPI from "near-api-js"
import { config } from "./config";
import axios from 'axios';

const {
    FUNDING_DATA,
    FUNDING_DATA_BACKUP,
    ACCOUNT_LINKS,
    GAS,
    SEED_PHRASE_LOCAL_COPY,
    networkId,
    nodeUrl,
    walletUrl,
    nameSuffix,
    contractName
} = config;


class NearStore {
    @observable logged: boolean;
    @observable token: string;
    @observable currentUser: User;
    @observable processing: boolean;
    @observable near: Object;

    constructor() {
        this.logged = false;
        this.token = '';
        this.currentUser = null;
        this.processing = false;
        const logged = localStorage.getItem('Logged');
        const token = localStorage.getItem('Token');
        const currentUser = localStorage.getItem('CurrentUser');
        if (logged) {
            this.logged = JSON.parse(logged);
        }
        if (token) {
            this.token = JSON.parse(token);
        }
        if (currentUser) {
            this.currentUser = JSON.parse(currentUser);
        }

        this.near = await nearAPI.connect({
            networkId, nodeUrl, walletUrl, deps: { keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore() },
        });
    }

    async isAccountTaken(accountId) {
        const account = new nearAPI.Account(this.near.connection, accountId);
        try {
            await account.state()
        } catch (e) {
            console.warn(e)
            if (/does not exist while viewing/.test(e.toString())) {
                return false
            }
        }
        return true
    }

    //     // resume wallet / contract flow
    const wallet = new nearAPI.WalletAccount(near);
    wallet.signIn = () => {
        wallet.requestSignIn(contractName, 'Blah Blah')
    }
    wallet.signedIn = wallet.isSignedIn()
    if(wallet.signedIn) {
        wallet.balance = formatNearAmount((await wallet.account().getAccountBalance()).available, 2)
    }

    const contract = await new nearAPI.Contract(wallet.account(), contractName, {
        changeMethods: ['send', 'create_account', 'create_account_and_claim'],
    })
    wallet.isAccountTaken = async (accountId) => {
        const accountTaken = await isAccountTaken(accountId + nameSuffix)
        update('app', { accountTaken, wasValidated: true })
    }
    wallet.fundAccount = async (amount, accountId, recipientName) => {
        if (accountId.indexOf(nameSuffix) > -1 || accountId.indexOf('.') > -1) {
            alert(nameSuffix + ' is added automatically and no "." is allowed. Please remove and try again.')
            return update('app.wasValidated', true)
        }
        accountId = accountId + nameSuffix
        if (parseFloat(amount, 10) < 0.1 || accountId.length < 2 || accountId.length > 48) {
            return update('app.wasValidated', true)
        }
        const keyPair = KeyPair.fromRandom('ed25519')

        const links = get(ACCOUNT_LINKS, [])
        links.push({ key: keyPair.secretKey, accountId, recipientName, keyStored: Date.now() })
        set(ACCOUNT_LINKS, links)

        // set(FUNDING_DATA, { key: keyPair.secretKey, accountId, recipientName, amount, funder_account_id: wallet.getAccountId() })
        await contract.create_account({ new_account_id: accountId, new_public_key: keyPair.publicKey.toString() }, GAS, parseNearAmount(amount))
    }

}
export default new AuthStore();
