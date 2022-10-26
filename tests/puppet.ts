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
  const puppetKeypair = Keypair.generate();

  it("Does CPI", async () => {

    await puppetProgram.methods
        .initialize()
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
            puppetData: puppetKeypair.publicKey
          })
          .rpc();
    
    let puppetDataAccount = await puppetProgram.account.data.fetch(
        puppetKeypair.publicKey);

    expect(puppetDataAccount.data.toNumber()).to.equal(15);

  });
    
  

});
