import * as anchor from "@project-serum/anchor";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { expect } from "chai";
import {
  awaitTransaction,
  getPda,
  getProgram,
  getRandomPayer,
} from "../base_test";
import {
  ALLOVR_AOVR_STAKE_NUM_POOLS,
  ALLOVR_AOVR_STAKE_NUM_STAKES_IN_POOL,
  ALLOVR_AOVR_STAKE_POOL_PREFIX,
  ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX,
  COST_REGISTER_POOL,
} from "../constants";

const tryRegister = async (
  stakePoolIndex: number,
  pdaPrefix = ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX
): Promise<{
  txSignature?: string;
  didFail: boolean;
  stakePoolPda?: anchor.web3.PublicKey;
  stakePoolRegistryPda?: anchor.web3.PublicKey;
}> => {
  const program = getProgram();
  const payer = await getRandomPayer(COST_REGISTER_POOL);
  const stakePoolRegistryPda = await getPda([utf8.encode(pdaPrefix)]);

  const stakePoolPda = await getPda([
    utf8.encode(ALLOVR_AOVR_STAKE_POOL_PREFIX),
    payer.publicKey.toBuffer(),
    new Uint8Array([stakePoolIndex]),
  ]);

  try {
    const txSignature = await program.methods
      .registerStakingPool(stakePoolIndex)
      .accounts({
        stakePool: stakePoolPda,
        stakePoolRegistry: stakePoolRegistryPda,
        initialiser: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .rpc();

    return { txSignature, didFail: false, stakePoolPda, stakePoolRegistryPda };
  } catch {
    return { didFail: true };
  }
};

const registerAndConfirmValues = async (stakePoolIndex: number) => {
  const program = getProgram();
  try {
    const { didFail, txSignature, stakePoolPda, stakePoolRegistryPda } =
      await tryRegister(stakePoolIndex);

    expect(didFail).false;

    await awaitTransaction(txSignature);

    const stakePool = await program.account.stakePool.fetch(stakePoolPda);
    expect(stakePool.staked.toNumber()).to.eq(0);
    expect(stakePool.stakes).to.length(ALLOVR_AOVR_STAKE_NUM_STAKES_IN_POOL);
    expect(stakePool.stakes.every((s) => s.eqn(0))).true;

    const stakePoolRegistry = await program.account.stakePoolRegistry.fetch(
      stakePoolRegistryPda
    );

    expect(stakePoolRegistry.totalStaked.toNumber()).to.eq(0);
    expect(stakePoolRegistry.totalOwed.toNumber()).to.eq(0);
    expect(stakePoolRegistry.poolHead).to.eq(stakePoolIndex + 1);
    expect(stakePoolRegistry.pools).to.length(ALLOVR_AOVR_STAKE_NUM_POOLS);

    const allPoolSlotsButFirst = (stakePoolRegistry.pools as Array<any>).slice(
      stakePoolIndex + 1
    );
    expect(allPoolSlotsButFirst.every((s) => s === null)).true;

    const stakePoolInfo = stakePoolRegistry.pools[stakePoolIndex];
    expect(stakePoolInfo.totalStaked.toNumber()).to.eq(0);
    expect(stakePoolInfo.totalOwed.toNumber()).to.eq(0);
    expect(stakePoolPda.equals(stakePoolInfo.poolAddress)).true;
  } catch (e) {
    console.error("Failed to register staking pool");
    console.error(e);
    throw e;
  }
};

describe("Register Staking Pool", () => {
  it(`Cannot register at incorrect index 10`, async () => {
    const { didFail } = await tryRegister(10);
    expect(didFail).true;
  });

  it(`Registers staking pool at index 0`, async () => {
    await registerAndConfirmValues(0);
  });

  it(`Cannot register at same index 0 again`, async () => {
    const { didFail } = await tryRegister(0);
    expect(didFail).true;
  });

  it(`Registers staking pool at index 1`, async () => {
    await registerAndConfirmValues(1);
  });
});
