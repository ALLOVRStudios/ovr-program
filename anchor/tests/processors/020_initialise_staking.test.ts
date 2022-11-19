import * as anchor from "@project-serum/anchor";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { assert, expect } from "chai";
import {
  awaitTransaction,
  getPda,
  getProgram,
  getRandomPayer,
} from "../base_test";
import {
  ALLOVR_AOVR_STAKE_NUM_POOLS,
  ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX,
  COST_INITIALISE,
} from "../constants";

describe("Initialise staking", () => {
  const program = getProgram();
  it("Cannot initialis with wrong PDA", async () => {
    const initialiser = await getRandomPayer(COST_INITIALISE);
    const stakePoolRegistryPda = await getPda([utf8.encode("somejunk")]);

    let didFail = false;
    try {
      await program.methods
        .initialiseStaking()
        .accounts({
          stakePoolRegistry: stakePoolRegistryPda,
          initialiser: initialiser.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([initialiser])
        .rpc();
    } catch (e) {
      didFail = true;
    }

    expect(didFail).true;

    let accountDoesNotExist = false;
    try {
      await program.account.stakePoolRegistry.fetch(stakePoolRegistryPda);
    } catch {
      accountDoesNotExist = true;
    }

    expect(accountDoesNotExist).true;
  });

  it("Initialises correctly", async () => {
    const initialiser = await getRandomPayer(COST_INITIALISE);
    const stakePoolRegistryPda = await getPda([
      utf8.encode(ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX),
    ]);

    try {
      const tx = await program.methods
        .initialiseStaking()
        .accounts({
          stakePoolRegistry: stakePoolRegistryPda,
          initialiser: initialiser.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([initialiser])
        .rpc();

      const response = await awaitTransaction(tx);
      if (response.value.err) {
        console.error(response.value.err);
        throw response.value.err;
      }
    } catch (e) {
      console.error("Failed to initialise staking");
      console.error(e);
      throw e;
    }

    const stakePoolRegistry = await program.account.stakePoolRegistry.fetch(
      stakePoolRegistryPda
    );
    expect(stakePoolRegistry.totalStaked.toNumber()).to.eq(0);
    expect(stakePoolRegistry.totalOwed.toNumber()).to.eq(0);
    expect(stakePoolRegistry.poolHead).to.eq(0);
    expect(stakePoolRegistry.pools).to.length(ALLOVR_AOVR_STAKE_NUM_POOLS);
  });

  it("Cannot initialise twice", async () => {
    const initialiser = await getRandomPayer(COST_INITIALISE);
    const stakePoolRegistryPda = await getPda([
      utf8.encode(ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX),
    ]);

    let didFail = false;

    try {
      await program.methods
        .initialiseStaking()
        .accounts({
          stakePoolRegistry: stakePoolRegistryPda,
          initialiser: initialiser.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([initialiser])
        .rpc();
    } catch (e) {
      didFail = true;
    }

    expect(didFail).true;

    const stakePoolRegistry = await program.account.stakePoolRegistry.fetch(
      stakePoolRegistryPda
    );
    expect(stakePoolRegistry.totalStaked.toNumber()).to.eq(0);
    expect(stakePoolRegistry.totalOwed.toNumber()).to.eq(0);
    expect(stakePoolRegistry.poolHead).to.eq(0);
    expect(stakePoolRegistry.pools).to.length(ALLOVR_AOVR_STAKE_NUM_POOLS);
    expect((stakePoolRegistry.pools as Array<any>).every((s) => s === null))
      .true;
  });
});
