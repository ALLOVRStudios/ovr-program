import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { createAssociatedTokenAccountInstruction } from "@solana/spl-token";
import { OvrProgram } from "../target/types/ovr_program";
import {
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
  const [stakePoolRegistryPda] = await anchor.web3.PublicKey.findProgramAddress(
    seeds,
    programId
  );

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

export const getFounders = () => ({
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

  console.log("Creating ATA");

  // Fires a list of instructions
  const mint_tx = new anchor.web3.Transaction().add(
    createAssociatedTokenAccountInstruction(
      treasury.publicKey,
      trreasuryAta,
      treasury.publicKey,
      mint.publicKey
    )
  );

  // sends and create the transaction
  const res = await program.provider.sendAndConfirm(mint_tx, [treasury]);

  console.log("This is the respo");
  console.log(res);
};
