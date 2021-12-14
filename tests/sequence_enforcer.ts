import * as anchor from '@project-serum/anchor';
import {assert} from 'chai';
import { Program } from '@project-serum/anchor';
import { SequenceEnforcer } from '../target/types/sequence_enforcer';

describe('sequence_enforcer', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SequenceEnforcer as Program<SequenceEnforcer>;


  it('Initialize and reset, then increase', async () => {
    const [address, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("SOL-PERP")], program.programId)

    const tx = await program.rpc.initialize(bump, "SOL-PERP", {
      accounts: {
        sequenceAccount: address,
        authority: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      }
    });

    await program.rpc.resetSequenceNumber(new anchor.BN(1234), {
      accounts: {
        sequenceAccount: address,
        authority: program.provider.wallet.publicKey
      }
    });

    await program.rpc.checkAndSetSequenceNumber(new anchor.BN(1235), {
      accounts: {
        sequenceAccount: address,
        authority: program.provider.wallet.publicKey
      }
    });

    //the following line can be uncommented to attach a debugger in visual studio
    //debugger;

    console.log("Your transaction signature", tx);
  });

  it('Increase out of order', async () => {
    const [address, bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("SOL-PERP")], program.programId)

    await program.rpc.checkAndSetSequenceNumber(new anchor.BN(1237), {
      accounts: {
        sequenceAccount: address,
        authority: program.provider.wallet.publicKey
      }
    });

    try {
      await program.rpc.checkAndSetSequenceNumber(new anchor.BN(1236), {
        accounts: {
          sequenceAccount: address,
          authority: program.provider.wallet.publicKey
        }
      });
    }
    catch(e) {
      assert(e.msg, 'Sequence out of order');
      return;
    }

    console.assert(false);
  });
});
