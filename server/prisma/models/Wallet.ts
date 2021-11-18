import { User } from "./User";

export type Wallet = {
    id: bigint,
    wallet: string,
    public_key: string,
    seed_phrase: string,
    user: User,
    userId: bigint
}