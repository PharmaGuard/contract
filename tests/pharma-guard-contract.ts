import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PharmaGuardContract } from "../target/types/pharma_guard_contract";
import { assert, expect } from "chai";

describe("pharma-guard-contract", () => {
  // Configure the client to use the local cluster.
  // anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PharmaGuardContract as Program<PharmaGuardContract>;

  const new_medication = {
    name: "Paracetamol",
    manufacturer: "Pfizer",
    temperature: 25,
  };

  const [medication_pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("medication"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("creates a new medication", async () => {
    // Add your test here.
    const tx = await program.methods.createMedication(new_medication).accounts({
      medication: medication_pda,
    }).rpc();
    console.log("Your transaction signature", tx);

    const medication = await program.account.medicationData.fetch(medication_pda);
    expect(medication.name === new_medication.name);
    expect(medication.manufacturer === new_medication.manufacturer);
    expect(medication.temperature === new_medication.temperature);
  });
});
