use crate::*;

pub trait UpgradableNFT {
    fn upgrade(&mut self, account_id :ValidAccountId,series_id:NftSeriesId)->Promise;
}

/*
--30 NFT Series

Series1 1000 copies
Series2 1000 copies
Series2 1000 copies

User1
Series1/copy1

User2
Series1/copy2

--Upgrade for user 1, but only user has a copy from series1

--After Update:
User1
Series2/copy1

*/