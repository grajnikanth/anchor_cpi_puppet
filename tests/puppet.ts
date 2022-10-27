import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Puppet } from "../target/types/puppet";
import { Keypair } from '@solana/web3.js';
import { expect } from 'chai';
import { PuppetMaster } from '../traget/types/puppet_master';

describe("puppet", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const puppetProgram = anchor.workspace.Puppet as Program<Puppet>;
  const puppetMasterProgram = anchor.workspace.PuppetMaster as Program<PuppetMaster>;

  // puppetKeypair will be the address where a new account will be created
  // using puppet program

  // authorityKeypair will be stored in the data and for changing the data, the 
  // authorityKeypair has to sign the transaction
  const puppetKeypair = Keypair.generate();
  const authorityKeypair = Keypair.generate();

  // The initialize function is being called directly on puppet smart contract
  // where the authority publickey is required as an argument to the initialize() function
  // on the smart contract. So below we are sending the authorityKyepair as an argument
  // when calling the initialize functon on puppet in the first transaction.
  // Here authority is not an account because, solana does not have to do anything with the
  // the authority account, all we are doing is storing the public key into the data. So
  // in this kind of cases, we should send the publickeys as arguments not as accounts.

  // In the second transaction below, we are calling the pullstring functon on the puppet_master
  // and in this case, the smart contract is expecting the authority signature so it is treated
  // as an account which the smart cotnract will interact with so in this transaction the 
  // authority publicKey has to be sent as an account instead of as an argument to the function
  // so that signatures from authority can be sent properly and that's how solana works when it deals
  // with accounts. 

  // In the second function call the puppet_master contract is able to extend the signature of the 
  // authority keypair to the puppet smart contract when making the CPI call. 

  // In the puppet_master pullstrings function call, the authority Keypair has to sign
  // this transaction

  it("Does CPI", async () => {

    await puppetProgram.methods
        .initialize(authorityKeypair.publicKey)
        .accounts({
          puppet: puppetKeypair.publicKey,
          user: provider.wallet.publicKey
        })
        .signers([puppetKeypair])
        .rpc();
    

    await puppetMasterProgram.methods
          .pullStrings(new anchor.BN(15))
          .accounts({
            puppetProgram: puppetProgram.programId,
            puppetData: puppetKeypair.publicKey,
            authority: authorityKeypair.publicKey
          }).
          signers([authorityKeypair])
          .rpc();
    
    let puppetDataAccount = await puppetProgram.account.data.fetch(
        puppetKeypair.publicKey);

    expect(puppetDataAccount.data.toNumber()).to.equal(15);

  });
    
  

});
