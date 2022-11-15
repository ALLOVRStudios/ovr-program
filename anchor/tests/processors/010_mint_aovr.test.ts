import * as anchor from "@project-serum/anchor";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { expect } from "chai";
import {
  allovrAovrTreasury,
  allovrMintKey,
  allovrSolTreasury,
  allovrStateKey,
} from "../test-keys/test-keys";
import {
  awaitTransaction,
  getFounders,
  getPda,
  getProgram,
  getRandomPayer,
  initialiseAllovrTreasury,
} from "../base_test";
import { ALLOVR_MINT_SEED_PREFIX, COST_INIT_AOVR } from "../constants";
import {
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

const tryMint = async (): Promise<{
  success: boolean;
  allovrStatePubkey?: anchor.web3.PublicKey;
  error?: any;
}> => {
  const program = getProgram();
  const payer = await getRandomPayer(COST_INIT_AOVR);
  const allovrStateKeypair = allovrStateKey();
  const allovrMintKeypair = allovrMintKey();
  const allovrSolTreasuryKeypair = allovrSolTreasury();
  const allovrAovrTreasuryAta = await allovrAovrTreasury();

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

    return { success: true, allovrStatePubkey: allovrStateKeypair.publicKey };
  } catch (error) {
    console.error(error);
    return { success: false, error };
  }
};

describe("Mint AOVR", () => {
  it(`Mints`, async () => {
    const program = getProgram();
    const founders = getFounders();
    await initialiseAllovrTreasury();
    const { success, allovrStatePubkey } = await tryMint();
    expect(success).true;
    const allovrAovrTreasuryAta = await allovrAovrTreasury();
    const y = await program.provider.connection.getTokenAccountBalance(
      allovrAovrTreasuryAta
    );

    console.log("This is the AOVR treasury");
    console.log(y);
  });
});
