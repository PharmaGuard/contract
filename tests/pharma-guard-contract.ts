import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PharmaGuardContract } from "../target/types/pharma_guard_contract";
import { assert, expect } from "chai";

describe("pharma-guard-contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PharmaGuardContract as Program<PharmaGuardContract>;

  const new_medication = {
    temperature: 25,
    expiration_date: new anchor.BN(1730085578),
  };

  const manufacturerKP = anchor.web3.Keypair.generate();

  it("creates a new medication", async () => {
    await program.methods.initializedManufacturer().accounts({
      manufacturer: manufacturerKP.publicKey,
    }).rpc();
    const manufacturer = await program.account.manufacturerData.fetch(manufacturerKP.publicKey);
    console.log("creates a new manufacturer", manufacturer);
  });
});
