import {Wallet} from "./Wallet";

//----------------------------------------------------------------------------------------
// from start we should register (seed) admin account like initial owner of all NFTs
//----------------------------------------------------------------------------------------
export enum Role {
    USER,
    ADMIN
}

export class User {
    id: null | BigInt = null;
    name: null | string = null;
    email: null | string = null;
    phone: null | string = null;
    role: Role = Role.USER;
    NearWallets: null | Wallet[] = null;
//    Nfts: Nft[]
}
