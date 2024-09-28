import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PharmaGuardContract } from "../target/types/pharma_guard_contract";
import { assert, expect } from "chai";

describe("pharma-guard-contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PharmaGuardContract as Program<PharmaGuardContract>;

  const new_medication = {
    name: "Paracetamol",
    manufacturer: "Pfizer",
    temperature: 25,
  };

  const [medication_pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(new_medication.name), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("creates a new medication", async () => {
    await program.methods.createMedication(new_medication.name, new_medication.manufacturer, new_medication.temperature).accounts({
      medication: medication_pda,
    }).rpc();

    const medication = await program.account.medicationData.fetch(medication_pda);
    expect(medication.name === new_medication.name);
    expect(medication.manufacturer === new_medication.manufacturer);
    expect(medication.temperature === new_medication.temperature);
  });

  it("update a medication", async () => {
    const newManufacturer = "9";
    const newTemperature = 9;

    await program.methods.updateMedication(new_medication.name, newManufacturer, newTemperature).accounts({
      medication: medication_pda,
    }).rpc();

    const medication = await program.account.medicationData.fetch(medication_pda);
    console.log(medication);

    expect(medication.manufacturer === newManufacturer);
    expect(medication.temperature === newTemperature);
  });

  it("delete a medication", async () => {
    await program.methods.deleteMedication(new_medication.name).accounts({
      medication: medication_pda,
    }).rpc();
  });
});
