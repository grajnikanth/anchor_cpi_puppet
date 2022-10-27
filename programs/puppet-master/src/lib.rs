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




// cpi conext preparation is handles in the set_data_ctx function implemented
// on the PullStrings struct below

// once we get the CpiContext we can now call the puppet set_data function
// from the puppet_master program

// The set_data function returns Ok(())

// added bump to simlate the PDA signature by this program when making call to 
// the puppet program. bump will be used to create the signature by this program
// bump will be pre-generate in client program. The other seed will be empty [] and
// only the bump seed with this programID will be used to get the hash of the
// authority account address.

// with_signer(seeds) function will add the signature fro the authority account 
// we are taking the CPIContext returned by the set_data_ctx() function and
// adding the signature here.
// There are two fields in the accounts here puppet and authority. It appears
// that anchor can figure out which of these accounts we want to sign and 
// anchor knows to pick the authority in this case maybe because the 
// puppet program says "authority" is the signer in the SetData validtaion
// struct? Not  sure how anchor knows which account it should sign among the two


    pub fn pull_strings(ctx: Context<PullStrings>, bump: u8, data: u64) -> Result<()> {
        let bump = &[bump][..];
        puppet::cpi::set_data(ctx.accounts.set_data_ctx().with_signer(&[&[bump][..]]), data)
    }
}

// Vaidation struct PullStrings for this program function pull_strings

// puppet_data is the struct corresponding to the puppet program "Data" struct
// puppet_program represents the puppet program as a struct with the anchor 
// Program wrapper

// Program<> wrapper validates that the account "Puppet" sent to the instruction
// is a program. This wrapper checks that the address sent matches the address
// of the "puppet" program and it also checks if the account_info.executable == true
//      i.e checks puppet_program.programId == programId when anchor created the CPI program
//      and account_info.executable == true

// authority will be set as an unchecked account because the client will not send
// the signature when invoking this instruction. The authority address will be 
// still in the accounts list sent by client but it will not signed in the client.
// The signature for authority will be provided by this program using the program
// signature as the authority will be a PDA.

// UncheckedAccount<> wrapper ensures that anchor will not do signature validation
// when the client sends the account to this program

// make sure to add teh comment "/// CHECK: ..." in the struct below above the 
// authority field, without this anchor will give an error when compiling

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet_data: Account<'info, Data>,
    pub puppet_program: Program<'info, Puppet>,
    /// CHECK: only used as a signing PDA
    pub authority: UncheckedAccount<'info>
}

// implement the set_data_ctx function as a function on the PullStrings struct
// to prepare the cpi call to puppet program. This will keep the business logic 
// in pull_strings function separate from cpi preparation logic

// self here is the PullStrings struct. The ctx.accounts sent to the 
// instruction by client is also same as PullStrings struct

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

// Note that in the cpi_accounts the puppet field does not have to sign this transaction
// to the puppet program. This puppet field is needed only to ensure that the
// puppet program knows the address of the account we want to edit.

// The puppet program does expect the authority to sign this transaction per
// the logic in the puppet program.

// See notes above the pullstrings function on how anchor determines which account to
// sign

impl<'info> PullStrings<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.puppet_program.to_account_info();    
        let cpi_accounts = SetData {
            puppet: self.puppet_data.to_account_info(),
            authority: self.authority.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}
