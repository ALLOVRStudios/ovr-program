import * as anchor from "@project-serum/anchor";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { expect } from "chai";
import {
  allovrAovrTreasury,
  allovrMintKey,
  allovrStateKey,
} from "../test-keys/test-keys";
import {
  awaitTransaction,
  checkMint,
  getPda,
  getProgram,
  getRandomPayer,
} from "../base_test";
import {
  ALLOVR_AOVR_DECIMAL_PLACES,
  ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX,
  ALLOVR_AOVR_STAKE_TREASURY_PREFIX,
  ALLOVR_MINT_SEED_PREFIX,
  COST_INIT_AOVR,
  INFLATION_INTERVAL_IN_SECONDS,
} from "../constants";
import { getMint, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const tryInflationRun = async (): Promise<{
  success: boolean;
  allovrStatePubkey: anchor.web3.PublicKey;
  allovrAovrTreasuryAta: anchor.web3.PublicKey;
  allovrMint: anchor.web3.PublicKey;
  error?: any;
}> => {
  const program = getProgram();
  const payer = await getRandomPayer(COST_INIT_AOVR);
  const allovrStateKeypair = allovrStateKey();
  const allovrMintKeypair = allovrMintKey();
  const allovrAovrTreasuryAta = await allovrAovrTreasury();
  const aovrStakingTreasuryPda = await getPda([
    utf8.encode(ALLOVR_AOVR_STAKE_TREASURY_PREFIX),
  ]);
  const stakePoolRegistryPda = await getPda([
    utf8.encode(ALLOVR_AOVR_STAKE_POOL_REGISTRY_PREFIX),
  ]);

  let success = true;
  let error = undefined;

  try {
    const mintAuthorityPda = await getPda([
      utf8.encode(ALLOVR_MINT_SEED_PREFIX),
    ]);

    const txSignature = await program.methods
      .aovrInflationRun()
      .accounts({
        aovrState: allovrStateKeypair.publicKey,
        aovrMint: allovrMintKeypair.publicKey,
        mintAuthority: mintAuthorityPda,
        aovrTreasury: allovrAovrTreasuryAta,
        aovrStakingTreasury: aovrStakingTreasuryPda,
        stakePoolRegistry: stakePoolRegistryPda,
        payer: payer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .signers([payer])
      .rpc();

    await awaitTransaction(txSignature);
  } catch (e) {
    success = false;
    error = e;
  }

  return {
    success,
    allovrStatePubkey: allovrStateKeypair.publicKey,
    allovrAovrTreasuryAta,
    allovrMint: allovrMintKeypair.publicKey,
    error,
  };
};

const scheduleInflationRunForNow = async (): Promise<{
  success: boolean;
  allovrStatePubkey: anchor.web3.PublicKey;
}> => {
  const program = getProgram();
  const allovrStateKeypair = allovrStateKey();
  let success = true;

  try {
    const txSignature = await program.methods
      .testUpdateInflationRun()
      .accounts({
        aovrState: allovrStateKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();

    await awaitTransaction(txSignature);
  } catch {
    success = false;
  }

  return {
    success,
    allovrStatePubkey: allovrStateKeypair.publicKey,
  };
};

describe("Inflation run", () => {
  const program = getProgram();
  it(`Fails as inflation run is not due`, async () => {
    const { success, allovrMint, error } = await tryInflationRun();

    expect(success).false;
    const mintInfo = await getMint(program.provider.connection, allovrMint);
    checkMint(mintInfo, "100000000000000000");

    expect(error.error.errorCode.code).eq("AovrInflationNotDue");
    expect(error.error.errorMessage).eq("AOVR inflation not due");
  });

  it(`Mints inflation tokens to DAO treasry`, async () => {
    const { success: resetSuccess, allovrStatePubkey } =
      await scheduleInflationRunForNow();
    expect(resetSuccess).true;

    const { success, allovrMint, allovrAovrTreasuryAta } =
      await tryInflationRun();

    expect(success).true;
    const mintInfo = await getMint(program.provider.connection, allovrMint);

    await checkMint(mintInfo, "100096153846153846");

    const allovrState = await program.account.allovrTokenState.fetch(
      allovrStatePubkey
    );

    expect(allovrState.inflationRunCount).eq(1);

    const allovrAovrTreasuryBalance =
      await program.provider.connection.getTokenAccountBalance(
        allovrAovrTreasuryAta
      );

    expect(allovrAovrTreasuryBalance.value.amount).eq("100096153846153846");
  });
});
