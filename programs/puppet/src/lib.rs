use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// this program instructions will be called by another Solana smart contract



#[program]
pub mod puppet {
    use super::*;

    // Initialize the data struct with authority field to store a 
    // pubkey in the data of the account
    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {
        ctx.accounts.puppet.authority = authority;
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        let puppet = &mut ctx.accounts.puppet;
        puppet.data = data;
        Ok(())
    }
}

// 

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + 8 + 32)]
    pub puppet: Account<'info, Data>,
    pub system_program: Program<'info, System>
}


// Added authority field to store the pubkey to whom this data belongs to
// and to provide that the signatures are transferred from client to puppet_master
// to puppet program via CPI

#[account]
pub struct Data {
    pub data: u64,
    pub authority: Pubkey
}


// has_one checks to see if
// puppet.authority = authority.keY()
// So the set_data function will only execute if the authority signs
// this transaction and authority is the same as that initially initialized in 
// data struct for that account

// authority has to sign this transaction becauase authority is assigned
// as Signer<> on this validation struct
#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut, has_one = authority)]
    pub puppet: Account<'info, Data>,
    pub authority: Signer<'info>
}