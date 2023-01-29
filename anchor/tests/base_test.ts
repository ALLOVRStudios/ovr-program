import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import {
  createAssociatedTokenAccountInstruction,
  Mint,
  uiAmountToAmount,
} from "@solana/spl-token";
import { expect } from "chai";
import { OvrProgram } from "../target/types/ovr_program";
import {
  ALLOVR_AOVR_DECIMAL_PLACES,
  ALLOVR_MINT_SEED_PREFIX,
  FOUNDER_1,
  FOUNDER_2,
  FOUNDER_3,
  FOUNDER_4,
  FOUNDER_5,
  FOUNDER_6,
  FOUNDER_7,
  FOUNDER_8,
} from "./constants";
import {
  allovrAovrTreasury,
  allovrMintKey,
  allovrSolTreasury,
} from "./test-keys/test-keys";

let program: anchor.Program<OvrProgram> | undefined;

export const getProgram = (): anchor.Program<OvrProgram> => {
  if (!program) {
    anchor.setProvider(anchor.AnchorProvider.env());
    program = anchor.workspace.OvrProgram as Program<OvrProgram>;
  }

  return program;
};

export const getRandomPayer = async (
  lamports: number
): Promise<anchor.web3.Keypair> => {
  const program = getProgram();
  const initialiser: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  const airdropSignature = await program.provider.connection.requestAirdrop(
    initialiser.publicKey,
    lamports
  );
  const latestBlockHash =
    await program.provider.connection.getLatestBlockhash();

  await program.provider.connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: airdropSignature,
  });

  return initialiser;
};

export const getPda = async (
  seeds: (Buffer | Uint8Array)[]
): Promise<anchor.web3.PublicKey> => {
  const programId = getProgram().programId;
  const [stakePoolRegistryPda, bump] =
    await anchor.web3.PublicKey.findProgramAddress(seeds, programId);

  return stakePoolRegistryPda;
};

export const awaitTransaction = async (
  txSignature: string
): Promise<anchor.web3.RpcResponseAndContext<anchor.web3.SignatureResult>> => {
  const program = getProgram();
  const latestBlockHash =
    await program.provider.connection.getLatestBlockhash();

  return await program.provider.connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: txSignature,
  });
};

export type FoundersList = {
  founder1: anchor.web3.PublicKey;
  founder2: anchor.web3.PublicKey;
  founder3: anchor.web3.PublicKey;
  founder4: anchor.web3.PublicKey;
  founder5: anchor.web3.PublicKey;
  founder6: anchor.web3.PublicKey;
  founder7: anchor.web3.PublicKey;
  founder8: anchor.web3.PublicKey;
};
export const getFounders = (): FoundersList => ({
  founder1: new anchor.web3.PublicKey(FOUNDER_1),
  founder2: new anchor.web3.PublicKey(FOUNDER_2),
  founder3: new anchor.web3.PublicKey(FOUNDER_3),
  founder4: new anchor.web3.PublicKey(FOUNDER_4),
  founder5: new anchor.web3.PublicKey(FOUNDER_5),
  founder6: new anchor.web3.PublicKey(FOUNDER_6),
  founder7: new anchor.web3.PublicKey(FOUNDER_7),
  founder8: new anchor.web3.PublicKey(FOUNDER_8),
});

export const initialiseAllovrTreasury = async () => {
  const mint = allovrMintKey();
  const treasury = allovrSolTreasury();
  const trreasuryAta = await allovrAovrTreasury();

  const airdropSignature = await program.provider.connection.requestAirdrop(
    treasury.publicKey,
    200_000_000
  );
  const latestBlockHash =
    await program.provider.connection.getLatestBlockhash();

  await program.provider.connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: airdropSignature,
  });

  const mint_tx = new anchor.web3.Transaction().add(
    createAssociatedTokenAccountInstruction(
      treasury.publicKey,
      trreasuryAta,
      treasury.publicKey,
      mint.publicKey
    )
  );

  await program.provider.sendAndConfirm(mint_tx, [treasury]);
};

export const checkFounderHaveNotChaged = (
  allovrState: any,
  founders: FoundersList
) => {
  expect(allovrState.founder1.equals(founders.founder1)).true;
  expect(allovrState.founder2.equals(founders.founder2)).true;
  expect(allovrState.founder3.equals(founders.founder3)).true;
  expect(allovrState.founder4.equals(founders.founder4)).true;
  expect(allovrState.founder5.equals(founders.founder5)).true;
  expect(allovrState.founder6.equals(founders.founder6)).true;
  expect(allovrState.founder7.equals(founders.founder7)).true;
  expect(allovrState.founder8.equals(founders.founder8)).true;
};

export const checkMint = async (mintInfo: Mint, supply: string) => {
  const mintAuthorityPda = await getPda([utf8.encode(ALLOVR_MINT_SEED_PREFIX)]);
  expect(mintInfo.mintAuthority.equals(mintAuthorityPda));
  expect(mintInfo.decimals).eq(ALLOVR_AOVR_DECIMAL_PLACES);
  expect(mintInfo.freezeAuthority).null;
  expect(mintInfo.isInitialized).true;
  expect(mintInfo.supply.toString()).eq(supply);
};

export const confirmInitialisedAllovrState = async (
  allovrState: any,
  founders: FoundersList,
  mintInfo: Mint
) => {
  checkFounderHaveNotChaged(allovrState, founders);
  expect(allovrState.inflationRunCount).eq(0);
  expect(allovrState.nextInflationDue.toNumber()).eq(0);
  expect(allovrState.minted).false;
  await checkMint(mintInfo, "0");
};

// const uiAmountToAmount = (uiAmount: number): BigInt => {
//   const amount =
//     BigInt(uiAmount) * BigInt(Math.pow(10, ALLOVR_AOVR_DECIMAL_PLACES));
//   console.log(`Converting ${uiAmount} to ${amount}`);
//   return amount;
// };
