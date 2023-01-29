import * as anchor from "@project-serum/anchor";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
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
  ALLOVR_AOVR_STAKE_TREASURY_PREFIX,
  ALLOVR_AUTHORITY_PREFIX,
  COST_INITIALISE,
} from "../constants";
import { allovrMintKey } from "../test-keys/test-keys";

type TryInitParams = {
  stakePoolRegistryPrefix?: string;
  stakeTreasuryPrefix?: string;
};

const tryInit = async ({
  stakePoolRegistryPrefix = ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX,
  stakeTreasuryPrefix = ALLOVR_AOVR_STAKE_TREASURY_PREFIX,
}: TryInitParams): Promise<{
  success: boolean;
  stakePoolRegistryPda: anchor.web3.PublicKey;
  aovrStakingTreasuryPda: anchor.web3.PublicKey;
}> => {
  const program = getProgram();
  const initialiser = await getRandomPayer(COST_INITIALISE);
  const stakePoolRegistryPda = await getPda([
    utf8.encode(stakePoolRegistryPrefix),
  ]);
  const aovrStakingTreasuryAuthorityPda = await getPda([
    utf8.encode(stakeTreasuryPrefix),
    utf8.encode(ALLOVR_AUTHORITY_PREFIX),
  ]);

  const aovrStakingTreasuryPda = await getPda([
    utf8.encode(stakeTreasuryPrefix),
  ]);

  const aovrMintKeypair = allovrMintKey();

  let success = true;
  try {
    await program.methods
      .initialiseStaking()
      .accounts({
        stakePoolRegistry: stakePoolRegistryPda,
        aovrMint: aovrMintKeypair.publicKey,
        aovrStakingTreasuryAuthority: aovrStakingTreasuryAuthorityPda,
        aovrStakingTreasury: aovrStakingTreasuryPda,
        initialiser: initialiser.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([initialiser])
      .rpc();
  } catch (e) {
    success = false;
  }

  return { success, stakePoolRegistryPda, aovrStakingTreasuryPda };
};

describe("Initialise staking", () => {
  const program = getProgram();
  it("Cannot initialise Stake Pool Registry with wrong PDA prefix", async () => {
    const { success, stakePoolRegistryPda, aovrStakingTreasuryPda } =
      await tryInit({
        stakePoolRegistryPrefix: "SomeJunk",
      });
    expect(success).false;

    let accountDoesNotExist = false;
    try {
      await program.account.stakePoolRegistry.fetch(stakePoolRegistryPda);
    } catch {
      accountDoesNotExist = true;
    }

    expect(accountDoesNotExist).true;
  });

  it("Cannot initialise Staking Treasury with wrong PDA prefix", async () => {
    const { success, aovrStakingTreasuryPda } = await tryInit({
      stakeTreasuryPrefix: "SomeJunk",
    });
    expect(success).false;

    let accountDoesNotExist = false;
    try {
      await program.provider.connection.getTokenAccountBalance(
        aovrStakingTreasuryPda
      );
    } catch {
      accountDoesNotExist = true;
    }

    expect(accountDoesNotExist).true;
  });

  it("Initialises correctly", async () => {
    const { success, aovrStakingTreasuryPda, stakePoolRegistryPda } =
      await tryInit({});
    expect(success).true;

    const stakePoolRegistry = await program.account.stakePoolRegistry.fetch(
      stakePoolRegistryPda
    );
    expect(stakePoolRegistry.totalStaked.toNumber()).to.eq(0);
    expect(stakePoolRegistry.poolHead).to.eq(0);
    expect(stakePoolRegistry.pools).to.length(ALLOVR_AOVR_STAKE_NUM_POOLS);
    expect(stakePoolRegistry.treasury.equals(aovrStakingTreasuryPda)).true;

    const treasuryBalance =
      await program.provider.connection.getTokenAccountBalance(
        aovrStakingTreasuryPda
      );
    expect(treasuryBalance.value.uiAmount).to.eq(0);

    console.log("--- Stake Pool Registry ---");
    console.table({
      totalStaked: stakePoolRegistry.totalStaked,
      poolHead: stakePoolRegistry.poolHead,
      treasury: stakePoolRegistry.treasury,
    });

    console.log("--- Stake Pool Pools ---");
    console.table(stakePoolRegistry.pools);
  });

  it("Cannot initialise twice", async () => {
    const { success, aovrStakingTreasuryPda, stakePoolRegistryPda } =
      await tryInit({});
    expect(success).false;

    const stakePoolRegistry = await program.account.stakePoolRegistry.fetch(
      stakePoolRegistryPda
    );
    expect(stakePoolRegistry.totalStaked.toNumber()).to.eq(0);
    expect(stakePoolRegistry.poolHead).to.eq(0);
    expect(stakePoolRegistry.pools).to.length(ALLOVR_AOVR_STAKE_NUM_POOLS);
    expect(stakePoolRegistry.treasury.equals(aovrStakingTreasuryPda)).true;

    const treasuryBalance =
      await program.provider.connection.getTokenAccountBalance(
        aovrStakingTreasuryPda
      );
    expect(treasuryBalance.value.uiAmount).to.eq(0);
  });
});
