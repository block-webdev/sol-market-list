use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::{ instruction::AuthorityType};

pub mod account;
pub mod constant;

use account::*;
use constant::*;


declare_id!("5kXnXwwEV6Abin9PqAs3PpDbAYuAAGPFhmRB6gxWRieb");

#[program]
pub mod sol_market_list {
    use super::*;
    
    
    pub fn initialize(ctx: Context<Initialize>, _bump : u8,) -> ProgramResult {
        msg!("initialize");

        let pool = &mut ctx.accounts.pool;
        pool.owner = *ctx.accounts.owner.key;
        pool.rand = *ctx.accounts.rand.key;
        pool.bump = _bump;

        Ok(())
    }

    pub fn add_nft(
        ctx : Context<AddNft>,
        nft_addr: Pubkey,
        owner: Pubkey,
        collection_id: u32,
        nft_id: u32,
        pool: Pubkey,
        ) -> ProgramResult {
        msg!("Add nft data");

        let nft_data = &mut ctx.accounts.nft_data;

        nft_data.nft_addr = nft_addr;
        nft_data.owner = owner;
        nft_data.collection_id = collection_id;
        nft_data.nft_id = nft_id;
        nft_data.pool = pool;

        Ok(())
    }

    pub fn remove_nft(
        ctx : Context<RemoveNft>,
        ) -> ProgramResult {
        msg!("Remove nft data");

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_bump : u8)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    owner : AccountInfo<'info>,

    #[account(init, payer = owner, seeds = [(*rand.key).as_ref()], bump = _bump)]
    pool : Account<'info, Pool>,

    rand : AccountInfo<'info>,

    system_program : Program<'info,System>,
}

#[derive(Accounts)]
pub struct AddNft<'info> {
    #[account(mut, signer)]
    owner : AccountInfo<'info>, 

    pool : Account<'info, Pool>,

    #[account(init_if_needed, payer=owner)]
    nft_data : Account<'info, NftData>,

    system_program : Program<'info,System>,
}


#[derive(Accounts)]
pub struct RemoveNft<'info> {
    #[account(mut, signer)]
    owner : AccountInfo<'info>, 

    pool : Account<'info, Pool>,

    #[account(mut, close = receiver)]
    nft_data : Account<'info, NftData>,

    #[account(mut)]
    pub receiver: SystemAccount<'info>
}