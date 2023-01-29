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

const tryInflationRun = async (): Promise<{
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
        payer: payer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .signers([payer])
      .rpc();

    await awaitTransaction(txSignature);

    return {
      success: true,
      allovrStatePubkey: allovrStateKeypair.publicKey,
      allovrAovrTreasuryAta,
      allovrMint: allovrMintKeypair.publicKey,
    };
  } catch (error) {
    console.error(error);
    return { success: false, error };
  }
};

describe.skip("Inflation run", () => {
  it(`Fails as inflation run is not ude`, async () => {
    const program = getProgram();
    const { success, allovrMint } = await tryInflationRun();

    expect(success).false;
    const mintInfo = await getMint(program.provider.connection, allovrMint);
    checkMint(mintInfo, 100_000_000);
  });
});
