import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PharmaGuardContract } from "../target/types/pharma_guard_contract";
import { assert, expect } from "chai";

describe("pharma-guard-contract", () => {
  const provider = anchor.AnchorProvider.env();
  const user_pk = provider.wallet.publicKey;
  console.log("user_pk", user_pk.toString());

  anchor.setProvider(provider);

  const program = anchor.workspace.PharmaGuardContract as Program<PharmaGuardContract>;

  const pharmacy = {
    license_number: 123456,
    phone_number: 132599,
  };

  const drug1 = {
    temperature: 23,
    batch_number: 1,
  };

  const drug2 = {
    temperature: 13,
    batch_number: 2,
  };

  const userAccount = anchor.web3.Keypair.generate();
  console.log("userAccount", userAccount.publicKey.toString());

  const pharmacyAccount = anchor.web3.Keypair.generate();
  console.log("pharmacyAccount", pharmacyAccount.publicKey.toString());

  const drugAccountA = anchor.web3.Keypair.generate();
  console.log("drugAccountA", drugAccountA.publicKey.toString());

  const drugAccountB = anchor.web3.Keypair.generate();
  console.log("drugAccountB", drugAccountB.publicKey.toString());

  before(async () => {
    await program.methods.initializedUser().accounts({
      user: userAccount.publicKey,
    }).signers([userAccount]).rpc();

    await program.methods.initialPharmacy(pharmacy.license_number, pharmacy.phone_number).accounts({
      pharmacyAccount: pharmacyAccount.publicKey,
    }).signers([pharmacyAccount]).rpc();

    await program.methods.initialDrug(drug1.temperature, drug1.batch_number).accounts({
      drugAccount: drugAccountA.publicKey,
    }).signers([drugAccountA]).rpc();

    await program.methods.initialDrug(drug2.temperature, drug2.batch_number).accounts({
      drugAccount: drugAccountB.publicKey,
    }).signers([drugAccountB]).rpc();
  });

  it("place order and sign for", async () => {
    console.log("place order");
    console.log("userAccount", userAccount.publicKey.toString());

    await program.methods.createOrder().accounts({
      user: userAccount.publicKey,
      pharmacy: pharmacyAccount.publicKey,
      drug: drugAccountA.publicKey,
    }).rpc();

    const user = await program.account.user.fetch(userAccount.publicKey);
    console.log("order info:", user.pharmacyAccountList[0].orderHistory[0]);

    await program.methods.sendOut().accounts({
      user: userAccount.publicKey,
      pharmacy: pharmacyAccount.publicKey,
      drug: drugAccountA.publicKey,
    }).rpc();


    await program.methods.signFor().accounts({
      user: userAccount.publicKey,
      pharmacy: pharmacyAccount.publicKey,
      drug: drugAccountA.publicKey,
    }).rpc();

    const user2 = await program.account.user.fetch(userAccount.publicKey);
    console.log("order info:", user2.pharmacyAccountList[0].orderHistory[0]);
  });


  it("place order and loss drug", async () => {
    console.log("loss drug");
    await program.methods.createOrder().accounts({
      user: userAccount.publicKey,
      pharmacy: pharmacyAccount.publicKey,
      drug: drugAccountB.publicKey,
    }).rpc();

    const user = await program.account.user.fetch(userAccount.publicKey);
    console.log("order info:", user.pharmacyAccountList[0].orderHistory[0]);

    await program.methods.sendOut().accounts({
      user: userAccount.publicKey,
      pharmacy: pharmacyAccount.publicKey,
      drug: drugAccountB.publicKey,
    }).rpc();

    await program.methods.lossDrug().accounts({
      user: userAccount.publicKey,
      pharmacy: pharmacyAccount.publicKey,
      drug: drugAccountB.publicKey,
    }).rpc();

    const user2 = await program.account.user.fetch(userAccount.publicKey);
    console.log("order info:", user2.pharmacyAccountList[0].orderHistory[0]);
  });
});
