import * as anchor from "@project-serum/anchor";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { expect } from "chai";
import { allovrMintKey, allovrStateKey } from "../test-keys/test-keys";
import {
  awaitTransaction,
  getFounders,
  getPda,
  getProgram,
  getRandomPayer,
} from "../base_test";
import {
  ALLOVR_AOVR_DECIMAL_PLACES,
  ALLOVR_MINT_SEED_PREFIX,
  COST_INIT_AOVR,
} from "../constants";
import { getMint, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const tryInitialise = async (): Promise<{
  success: boolean;
  allovrStatePubkey?: anchor.web3.PublicKey;
  allovrMintPubkey?: anchor.web3.PublicKey;
  allovrMintAuthorityPubkey?: anchor.web3.PublicKey;
  error?: any;
}> => {
  const program = getProgram();
  const payer = await getRandomPayer(COST_INIT_AOVR);
  const allovrStateKeypair = allovrStateKey();
  const allovrMintKeypair = allovrMintKey();

  try {
    const mintAuthorityPda = await getPda([
      utf8.encode(ALLOVR_MINT_SEED_PREFIX),
    ]);

    const txSignature = await program.methods
      .initialiseAovr(getFounders())
      .accounts({
        aovrState: allovrStateKeypair.publicKey,
        aovrMint: allovrMintKeypair.publicKey,
        mintAuthority: mintAuthorityPda,
        initialiser: payer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([payer, allovrStateKeypair, allovrMintKeypair])
      .rpc();

    await awaitTransaction(txSignature);

    return {
      success: true,
      allovrStatePubkey: allovrStateKeypair.publicKey,
      allovrMintPubkey: allovrMintKeypair.publicKey,
      allovrMintAuthorityPubkey: mintAuthorityPda,
    };
  } catch (error) {
    console.error(error);
    return { success: false, error };
  }
};

describe("Initialise AOVR", () => {
  const program = getProgram();
  const founders = getFounders();
  it(`Initialises`, async () => {
    const {
      success,
      allovrStatePubkey,
      allovrMintPubkey,
      allovrMintAuthorityPubkey,
    } = await tryInitialise();
    expect(success).true;

    const allovrState = await program.account.allovrTokenState.fetch(
      allovrStatePubkey
    );

    expect(allovrState.inflationRunCount).eq(0);
    expect(allovrState.nextInflationDue.toNumber()).eq(0);
    expect(allovrState.minted).false;
    expect(allovrState.founder1.equals(founders.founder1)).true;
    expect(allovrState.founder2.equals(founders.founder2)).true;
    expect(allovrState.founder3.equals(founders.founder3)).true;
    expect(allovrState.founder4.equals(founders.founder4)).true;
    expect(allovrState.founder5.equals(founders.founder5)).true;
    expect(allovrState.founder6.equals(founders.founder6)).true;
    expect(allovrState.founder7.equals(founders.founder7)).true;
    expect(allovrState.founder8.equals(founders.founder8)).true;

    const mintInfo = await getMint(
      program.provider.connection,
      allovrMintPubkey
    );

    expect(mintInfo.mintAuthority.equals(allovrMintAuthorityPubkey));
    expect(mintInfo.decimals).eq(ALLOVR_AOVR_DECIMAL_PLACES);
    expect(mintInfo.freezeAuthority).null;
    expect(mintInfo.isInitialized).true;
    expect(mintInfo.supply).eq(BigInt(0));
  });
});
