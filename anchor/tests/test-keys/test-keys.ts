import { readFileSync } from "fs";
import { join } from "path";
import { Keypair, PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { ALLOVR_AOVR_STAKE_TREASURY_PREFIX } from "../constants";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { getPda } from "../base_test";

const readKey = (fileName: string): Keypair => {
  const path = join(__dirname, fileName);
  const keyString = readFileSync(path, "utf-8");
  const key = Uint8Array.from(JSON.parse(keyString));
  const keypair = Keypair.fromSecretKey(key);
  return keypair;
};

export const allovrStateKey = (): Keypair => readKey("./allovr-state.json");

export const allovrMintKey = (): Keypair => readKey("./allovr-mint.json");

export const allovrSolTreasury = (): Keypair =>
  readKey("./allovr-aovr-treasury.json");

export const allovrAovrTreasury = async (): Promise<PublicKey> => {
  const ata = getAssociatedTokenAddress(
    allovrMintKey().publicKey,
    allovrSolTreasury().publicKey
  );

  return ata;
};
