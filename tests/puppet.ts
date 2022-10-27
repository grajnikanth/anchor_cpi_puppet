import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Puppet } from "../target/types/puppet";
import { Keypair, PublicKey } from '@solana/web3.js';
import { expect } from 'chai';
import { PuppetMaster } from '../traget/types/puppet_master';

// See git log to see the various evolution this code took as the 
// anchor concepts were developed.

describe("puppet", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const puppetProgram = anchor.workspace.Puppet as Program<Puppet>;
  const puppetMasterProgram = anchor.workspace.PuppetMaster as Program<PuppetMaster>;

  // puppetKeypair will be the address where a new account will be created
  // using puppet program

  const puppetKeypair = Keypair.generate();

  // The initialize function is being called directly on puppet smart contract
  // where the authority publickey is required as an argument to the initialize() function
  // on the smart contract. So below we are sending the authorityKyepair as an argument
  // when calling the initialize functon on puppet in the first transaction.
  // Here authority is not an account because, solana does not have to do anything with the
  // the authority account, all we are doing is storing the public key into the data. So
  // in this kind of cases, we should send the publickeys as arguments not as accounts.

  // Pre-determine the PDA with the puppetMaster program ID as the seed along
  // with the bump. This PDA will be called the authority address and send to
  // puppet program to initialize and store.

  // In the second function below the pullstring method needs the bump so that the
  // puppetMaster program can sign for authority when making the CPI call to the
  // set_data function of the puppet program. Authority need not sign this transaction
  // on the client side like what we did in the previous version of this code. See
  // previous git commit

  it("Does CPI", async () => {

    const [puppetMasterPDA, puppetMasterBump] = await PublicKey
        .findProgramAddress([], puppetMasterProgram.programId)

    await puppetProgram.methods
        .initialize(puppetMasterPDA)
        .accounts({
          puppet: puppetKeypair.publicKey,
          user: provider.wallet.publicKey
        })
        .signers([puppetKeypair])
        .rpc();
    

    await puppetMasterProgram.methods
          .pullStrings(puppetMasterBump, new anchor.BN(15))
          .accounts({
            puppetProgram: puppetProgram.programId,
            puppetData: puppetKeypair.publicKey,
            authority: puppetMasterPDA
          })
          .rpc();
    
    let puppetDataAccount = await puppetProgram.account.data.fetch(
        puppetKeypair.publicKey);

    expect(puppetDataAccount.data.toNumber()).to.equal(15);

  });
    
  

});
