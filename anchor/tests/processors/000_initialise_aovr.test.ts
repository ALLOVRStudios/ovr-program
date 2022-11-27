import * as anchor from "@project-serum/anchor";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { expect } from "chai";
import { allovrMintKey, allovrStateKey } from "../test-keys/test-keys";
import {
  awaitTransaction,
  confirmInitialisedAllovrState,
  getFounders,
  getPda,
  getProgram,
  getRandomPayer,
} from "../base_test";
import { ALLOVR_MINT_SEED_PREFIX, COST_INIT_AOVR } from "../constants";
import { getMint, Mint, TOKEN_PROGRAM_ID } from "@solana/spl-token";

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

    const mintInfo = await getMint(
      program.provider.connection,
      allovrMintPubkey
    );

    confirmInitialisedAllovrState(allovrState, founders, mintInfo);
  });

  it(`Fails to initialise a second time`, async () => {
    const { success } = await tryInitialise();
    expect(success).false;

    const allovrStateKeypair = allovrStateKey();

    const allovrState = await program.account.allovrTokenState.fetch(
      allovrStateKeypair.publicKey
    );

    // check nothing changed
    const allovrMintKeypair = allovrMintKey();

    const mintInfo = await getMint(
      program.provider.connection,
      allovrMintKeypair.publicKey
    );

    confirmInitialisedAllovrState(allovrState, founders, mintInfo);
  });
});
