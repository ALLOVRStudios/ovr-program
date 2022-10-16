import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { OvrProgram } from "../target/types/ovr_program";

export const getProgram = (): anchor.Program<OvrProgram> => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.OvrProgram as Program<OvrProgram>;
  return program;
};

export const getRandomPayer = async (
  lamports: number
): Promise<anchor.web3.Keypair> => {
  const program = getProgram();
  anchor.workspace;
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
