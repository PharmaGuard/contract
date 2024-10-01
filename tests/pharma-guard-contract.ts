import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PharmaGuardContract } from "../target/types/pharma_guard_contract";
import { assert, expect } from "chai";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js'
import { delay, safeAirdrop, getDrugInfo } from './utils/utils'
import { mintTo, createMint, getAssociatedTokenAddress, createAssociatedTokenAccount, getAccount, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, TokenAccountNotFoundError } from '@solana/spl-token'


const INITIAL: number = 1 << 0;
const IN_DELIVERY: number = 1 << 1;
const CANCELLED: number = 1 << 2;
const COMPLETED: number = 1 << 3;

const MEDICATION_NORMAL: number = 1 << 4;
const MEDICATION_LOST: number = 1 << 5;

describe("pharma-guard-contract", async () => {
  console.log("🚀 Starting tests...");
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.PharmaGuardContract as Program<PharmaGuardContract>;

  const pharmacy = {
    license_number: 123456,
    phone_number: 132599,
  };

  const drug1 = {
    price: new anchor.BN(99),
    temperature: 23,
    batch_number: 1,
  };

  const drug2 = {
    price: new anchor.BN(88),
    temperature: 13,
    batch_number: 2,
  };

  const userAccount = anchor.web3.Keypair.generate();
  console.log("userAccount", userAccount.publicKey.toString());

  const pharmaAccount = anchor.web3.Keypair.generate();
  console.log("pharmaAccount", pharmaAccount.publicKey.toString());

  const drugAccountA = anchor.web3.Keypair.generate();
  console.log("drugAccountA", drugAccountA.publicKey.toString());

  const drugAccountB = anchor.web3.Keypair.generate();
  console.log("drugAccountB", drugAccountB.publicKey.toString());

  let [userPharmacyAssociatedAccount,] = await PublicKey.findProgramAddressSync(
    [userAccount.publicKey.toBuffer(), Buffer.from("user_pharmacy_associated_account")],
    program.programId
  )
  console.log("userPharmacyAssociatedAccount", userPharmacyAssociatedAccount.toString());


  let [userATA,] = await PublicKey.findProgramAddressSync(
    [userAccount.publicKey.toBuffer(), Buffer.from("user_ATA")],
    program.programId
  )
  console.log("userATA", userATA.toString());

  let [pharmacyInfoAccount,] = await PublicKey.findProgramAddressSync(
    [pharmaAccount.publicKey.toBuffer(), Buffer.from("pharmacy_info_account")],
    program.programId
  )
  console.log("pharmacyInfoAccount", pharmacyInfoAccount.toString());

  let [pharmacyATA,] = await PublicKey.findProgramAddressSync(
    [pharmaAccount.publicKey.toBuffer(), Buffer.from("pharmacy_ATA")],
    program.programId
  )
  console.log("pharmacyATA", pharmacyATA.toString());

  let pharmaTokenMint;

  before(async () => {
    console.log("=== Airdropping tokens to user account ===");
    await safeAirdrop(userAccount.publicKey, provider.connection)
    await safeAirdrop(pharmaAccount.publicKey, provider.connection)
    delay(10000)

    pharmaTokenMint = await createMint(
      provider.connection,
      userAccount,
      userAccount.publicKey,
      undefined,
      6,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    )

    await program.methods.initialUserPaa().accounts({
      userPharmacyAssociatedAccount: userPharmacyAssociatedAccount,
      solAta: userATA,
      tokenMint: pharmaTokenMint,
      tokenProgram: TOKEN_PROGRAM_ID,
      authority: userAccount.publicKey,
    }).signers([userAccount]).rpc();


    await program.methods.initialPharmacy(pharmacy.license_number, pharmacy.phone_number).accounts({
      pharmacyInfoAccount: pharmacyInfoAccount,
      sol_ata: pharmacyATA,
      payer: pharmaAccount.publicKey,
      tokenMint: pharmaTokenMint,
      tokenProgram: TOKEN_PROGRAM_ID,
    }).signers([pharmaAccount]).rpc();

    await program.methods.bindPharmacy().accounts({
      userPharmacyAssociatedAccount: userPharmacyAssociatedAccount,
      pharmacyInfoAccount: pharmacyInfoAccount,
      authority: userAccount.publicKey,
    }).signers([userAccount]).rpc();



    await program.methods.initialDrug(drug1.price, drug1.temperature, drug1.batch_number).accounts({
      drugAccount: drugAccountA.publicKey,
    }).signers([drugAccountA]).rpc();

    await program.methods.initialDrug(drug2.price, drug2.temperature, drug2.batch_number).accounts({
      drugAccount: drugAccountB.publicKey,
    }).signers([drugAccountB]).rpc();


    console.log("=== Minting tokens to user account ===");
    await mintTo(
      provider.connection,
      userAccount,
      pharmaTokenMint,
      userATA,
      userAccount,
      1000,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    )

    await mintTo(
      provider.connection,
      userAccount,
      pharmaTokenMint,
      pharmacyATA,
      userAccount,
      1000,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    )

    let userAccountInfo = await getAccount(provider.connection, userATA);
    let pharmacyAccountInfo = await getAccount(provider.connection, pharmacyATA);

    expect(Number(userAccountInfo.amount)).to.equal(1000);
    expect(Number(pharmacyAccountInfo.amount)).to.equal(1000);
  });



  it("place order and sign for", async () => {
    console.log("===place order and sign for===");
    await program.methods.takeoutOrder().accounts({
      userPharmacyAssociatedAccount: userPharmacyAssociatedAccount,
      pharmacyInfoAccount: pharmacyInfoAccount,
      drug: drugAccountA.publicKey,
      authority: userAccount.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
    }).signers([userAccount]).rpc();

    program.addEventListener("takeOutEvent", (event) => {
      console.log("TakeOutEvent:", event);
    });

    const drugAOrderAfterTakeout = await getDrugInfo(program, userPharmacyAssociatedAccount, 0, 0);
    console.log("drugAOrderAfterTakeout:", drugAOrderAfterTakeout);
    expect(drugAOrderAfterTakeout.status).to.equal(INITIAL);
    expect(drugAOrderAfterTakeout.drugStatus).to.equal(MEDICATION_NORMAL);

    await program.methods.sendOut().accounts({
      userPharmacyAssociatedAccount: userPharmacyAssociatedAccount,
      drug: drugAccountA.publicKey,
      pharmacyInfoAccount: pharmacyInfoAccount,
      authority: pharmaAccount.publicKey,
    }).signers([pharmaAccount]).rpc();

    const drugAOrderAfterSendout = await getDrugInfo(program, userPharmacyAssociatedAccount, 0, 0);
    console.log("drugAOrderAfterSendout:", drugAOrderAfterSendout);
    expect(drugAOrderAfterSendout.status).to.equal(IN_DELIVERY);
    expect(drugAOrderAfterSendout.drugStatus).to.equal(MEDICATION_NORMAL);


    await program.methods.signFor().accounts({
      userPharmacyAssociatedAccount: userPharmacyAssociatedAccount,
      pharmacyInfoAccount: pharmacyInfoAccount,
      userSolAta: userATA,
      pharmacySolAta: pharmacyATA,
      drug: drugAccountA.publicKey,
      authority: userAccount.publicKey,
      tokenMint: pharmaTokenMint,
      tokenProgram: TOKEN_PROGRAM_ID,
    }).signers([userAccount]).rpc();

    const drugAOrderAfterSign = await getDrugInfo(program, userPharmacyAssociatedAccount, 0, 0);
    console.log("drugAOrderAfterSign:", drugAOrderAfterSign);
    expect(drugAOrderAfterSign.status).to.equal(COMPLETED);
    expect(drugAOrderAfterSign.drugStatus).to.equal(MEDICATION_NORMAL);

    let userAccountInfo = await getAccount(provider.connection, userATA);
    let pharmacyAccountInfo = await getAccount(provider.connection, pharmacyATA);
    expect(Number(userAccountInfo.amount)).to.equal(901);
    expect(Number(pharmacyAccountInfo.amount)).to.equal(1099);
  });


  it("place order and loss", async () => {
    console.log("===place order and sign for===");
    await program.methods.takeoutOrder().accounts({
      userPharmacyAssociatedAccount: userPharmacyAssociatedAccount,
      pharmacyInfoAccount: pharmacyInfoAccount,
      drug: drugAccountB.publicKey,
      authority: userAccount.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
    }).signers([userAccount]).rpc();

    program.addEventListener("takeOutEvent", (event) => {
      console.log("TakeOutEvent:", event);
    });

    const drugAOrderAfterTakeout = await getDrugInfo(program, userPharmacyAssociatedAccount, 0, 1);
    console.log("drugAOrderAfterTakeout:", drugAOrderAfterTakeout);
    expect(drugAOrderAfterTakeout.status).to.equal(INITIAL);
    expect(drugAOrderAfterTakeout.drugStatus).to.equal(MEDICATION_NORMAL);

    await program.methods.sendOut().accounts({
      userPharmacyAssociatedAccount: userPharmacyAssociatedAccount,
      drug: drugAccountB.publicKey,
      pharmacyInfoAccount: pharmacyInfoAccount,
      authority: pharmaAccount.publicKey,
    }).signers([pharmaAccount]).rpc();

    const drugAOrderAfterSendout = await getDrugInfo(program, userPharmacyAssociatedAccount, 0, 1);
    console.log("drugAOrderAfterSendout:", drugAOrderAfterSendout);
    expect(drugAOrderAfterSendout.status).to.equal(IN_DELIVERY);
    expect(drugAOrderAfterSendout.drugStatus).to.equal(MEDICATION_NORMAL);


    await program.methods.lossDrug().accounts({
      userPharmacyAssociatedAccount: userPharmacyAssociatedAccount,
      drug: drugAccountB.publicKey,
      pharmacyInfoAccount: pharmacyInfoAccount,
      authority: pharmaAccount.publicKey,
      tokenMint: pharmaTokenMint,
      tokenProgram: TOKEN_PROGRAM_ID,
    }).signers([pharmaAccount]).rpc();

    const drugAOrderAfterLoss = await getDrugInfo(program, userPharmacyAssociatedAccount, 0, 1);
    console.log("drugAOrderAfterLoss:", drugAOrderAfterLoss);
    expect(drugAOrderAfterLoss.status).to.equal(CANCELLED);
    expect(drugAOrderAfterLoss.drugStatus).to.equal(MEDICATION_LOST);
  });
});
