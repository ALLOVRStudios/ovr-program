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
} from "../constants";
import { OvrProgram } from "../../target/types/ovr_program";

describe("Register Staking Pool", () => {
  const program = getProgram();

  it("Registers staking pool", async () => {
    try {
      const stakePoolIndex = 0;
      const payer = await getRandomPayer(6681600);
      const stakePoolRegistryPda = await getPda([
        utf8.encode(ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX),
      ]);

      const stakePoolPda = await getPda([
        utf8.encode(ALLOVR_AOVR_STAKE_POOL_PREFIX),
        payer.publicKey.toBuffer(),
        new Uint8Array([stakePoolIndex]),
      ]);

      const registerPoolTxSignature = await program.methods
        .registerStakingPool(stakePoolIndex)
        .accounts({
          stakePool: stakePoolPda,
          stakePoolRegistry: stakePoolRegistryPda,
          initialiser: payer.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payer])
        .rpc();

      await awaitTransaction(registerPoolTxSignature);

      const stakePool = await program.account.stakePool.fetch(stakePoolPda);
      expect(stakePool.staked.toNumber()).to.eq(0);
      expect(stakePool.owed.toNumber()).to.eq(0);
      expect(stakePool.stakes).to.length(ALLOVR_AOVR_STAKE_NUM_STAKES_IN_POOL);
      expect(stakePool.stakes.every((s) => s.eqn(0))).true;

      const stakePoolRegistry = await program.account.stakePoolRegistry.fetch(
        stakePoolRegistryPda
      );

      expect(stakePoolRegistry.totalStaked.toNumber()).to.eq(0);
      expect(stakePoolRegistry.totalOwed.toNumber()).to.eq(0);
      expect(stakePoolRegistry.poolHead).to.eq(stakePoolIndex + 1);
      expect(stakePoolRegistry.pools).to.length(ALLOVR_AOVR_STAKE_NUM_POOLS);

      const allPoolSlotsButFirst = (
        stakePoolRegistry.pools as Array<any>
      ).slice(1);
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
  });
});
