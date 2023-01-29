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
  checkFounderHaveNotChaged,
  checkMint,
  getFounders,
  getPda,
  getProgram,
  getRandomPayer,
  initialiseAllovrTreasury,
} from "../base_test";
import {
  ALLOVR_AOVR_DECIMAL_PLACES,
  ALLOVR_MINT_SEED_PREFIX,
  COST_INIT_AOVR,
  INFLATION_INTERVAL_IN_SECONDS,
} from "../constants";
import { getMint, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const tryMint = async (): Promise<{
  success: boolean;
  allovrStatePubkey?: anchor.web3.PublicKey;
  allovrAovrTreasuryAta?: anchor.web3.PublicKey;
  allovrMint?: anchor.web3.PublicKey;
  error?: any;
}> => {
  const program = getProgram();
  const payer = await getRandomPayer(COST_INIT_AOVR);
  const allovrStateKeypair = allovrStateKey();
  const allovrMintKeypair = allovrMintKey();
  const allovrAovrTreasuryAta = await allovrAovrTreasury();
  let success = true;

  try {
    const mintAuthorityPda = await getPda([
      utf8.encode(ALLOVR_MINT_SEED_PREFIX),
    ]);

    const txSignature = await program.methods
      .mintAovr()
      .accounts({
        aovrState: allovrStateKeypair.publicKey,
        aovrMint: allovrMintKeypair.publicKey,
        mintAuthority: mintAuthorityPda,
        aovrTreasury: allovrAovrTreasuryAta,
        initialiser: payer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .signers([payer])
      .rpc();

    await awaitTransaction(txSignature);
  } catch (error) {
    success = false;
  }

  return {
    success,
    allovrStatePubkey: allovrStateKeypair.publicKey,
    allovrAovrTreasuryAta,
    allovrMint: allovrMintKeypair.publicKey,
  };
};

describe("Mint AOVR", () => {
  it(`Mints`, async () => {
    const program = getProgram();

    await initialiseAllovrTreasury();
    const { success, allovrStatePubkey, allovrAovrTreasuryAta, allovrMint } =
      await tryMint();

    expect(success).true;

    const allovrAovrTreasuryBalance =
      await program.provider.connection.getTokenAccountBalance(
        allovrAovrTreasuryAta
      );

    expect(allovrAovrTreasuryBalance.value.uiAmount).eq(100_000_000);
    expect(allovrAovrTreasuryBalance.value.decimals).eq(
      ALLOVR_AOVR_DECIMAL_PLACES
    );

    const allovrState = await program.account.allovrTokenState.fetch(
      allovrStatePubkey
    );

    checkFounderHaveNotChaged(allovrState, getFounders());

    const mintInfo = await getMint(program.provider.connection, allovrMint);
    await checkMint(mintInfo, "100000000000000000");

    expect(allovrState.minted).true;

    const unixTimestampNow = Math.floor(new Date().getTime() / 1000);
    const estimateNextInflationRun =
      unixTimestampNow + INFLATION_INTERVAL_IN_SECONDS;

    expect(
      allovrState.nextInflationDue.gt(
        new anchor.BN(estimateNextInflationRun - 10)
      )
    ).true;

    expect(
      allovrState.nextInflationDue.lt(new anchor.BN(estimateNextInflationRun))
    ).true;

    console.log("--- ALLOVR State ---");
    console.table(allovrState);

    console.log("--- ALLOVR AOVR Mint ---");
    console.table(mintInfo);
  });

  it(`Fails to mint a second time`, async () => {
    const program = getProgram();

    const { success, allovrStatePubkey, allovrAovrTreasuryAta, allovrMint } =
      await tryMint();

    expect(success).false;

    const allovrAovrTreasuryBalance =
      await program.provider.connection.getTokenAccountBalance(
        allovrAovrTreasuryAta
      );

    expect(allovrAovrTreasuryBalance.value.uiAmount).eq(100_000_000);

    const allovrState = await program.account.allovrTokenState.fetch(
      allovrStatePubkey
    );

    checkFounderHaveNotChaged(allovrState, getFounders());

    const mintInfo = await getMint(program.provider.connection, allovrMint);
    checkMint(mintInfo, "100000000000000000");

    expect(allovrState.minted).true;

    const unixTimestampNow = Math.floor(new Date().getTime() / 1000);
    const estimateNextInflationRun =
      unixTimestampNow + INFLATION_INTERVAL_IN_SECONDS;

    expect(
      allovrState.nextInflationDue.gt(
        new anchor.BN(estimateNextInflationRun - 10)
      )
    ).true;

    expect(
      allovrState.nextInflationDue.lt(new anchor.BN(estimateNextInflationRun))
    ).true;
  });
});
