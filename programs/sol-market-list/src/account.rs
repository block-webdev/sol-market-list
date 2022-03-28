
use anchor_lang::prelude::*;



#[account]
#[derive(Default)]
pub struct Pool {
    pub owner : Pubkey,
    pub rand : Pubkey,
    pub bump : u8,
}


#[account]
#[derive(Default)]
pub struct NftData {
    pub nft_addr: Pubkey,      // 32
    pub owner: Pubkey,      // 32

    // listed nft info for selling
    pub collection_id : u32,    // collection id containing nft
    pub nft_id : u32,   // nft id for selling

    // pool info
    pub pool : Pubkey,
}
