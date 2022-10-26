use anchor_lang::prelude::*;
// SetData is a validation struct in the puppet program
// cpi::accounts gets you a set of structs from the puppet program
// generated CPI struct
use puppet::cpi::accounts::SetData;
// Puppet is a struct representing the program
use puppet::program::Puppet;
// Data is regular struct inside puppet program
use puppet::{self, Data};

// update the puppet_master program_id
declare_id!("HmbTLCmaGvZhKnn1Zfa1JVnp7vkMV4DYVxPLWBVoN65L");

#[program]
pub mod puppet_master {
    use super::*;


    // prepare the Context for the puppet program in this function
    // Using the Context prepared here, we can then call the puppet program
    // via cross program invocation from this function to update the 
    // data in the Data struct

    // pull_strings will call the set_data function of the puppet program
    // To do CPI in anchor, the Conext shall include the accounts SetData 
    // validation struct required + the puppet program account to do CPI

    // client has to send in the publickey of the puppet program. which
    // will be validated by the PullStrings struct


    // anchor Context accounts field is Deserialized accounts. So looks like
    // when sending this accounts to the puppet program via cpi they have
    // to be converted to AccountInfo structs

    // cpi_program variable is set to the account_info struct by converting the
    // Program struct to account_info using to_account_info() function
    // This function was implemented on the 
    // anchor_lang::accounts::program::Program struct

    // The cpi invocation is syntax heavy in the sense that you have to follow
    // the anchor syntax for cpi invocation

    pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> Result<()> {
        
        let cpi_program = ctx.accounts.puppet_program.to_account_info();    
        let cpi_accounts = SetData {
            puppet: ctx.accounts.puppet_data.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        puppet::cpi::set_data(cpi_ctx, data);
    }
}

// Vaidation struct PullStrings for this program function pull_strings

// puppet_data is the struct corresponding to the puppet program "Data" struct
// puppet_program represents the puppet program as a struct with the anchor 
// Program wrapper

// Program<> wrapper validates that the account "Puppet" sent to the instruction
// is a program. This wrapper checks that the address sent matches the address
// of the "puppet" program and it also checks if the account_info.executable == true
//      i.e checks account_info.key == expected _program
//      and account_info.executable == true


#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet_data: Account<'info, Data>,
    pub puppet_program: Program<'info, Puppet>
}