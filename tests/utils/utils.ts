import { PublicKey, LAMPORTS_PER_SOL, Connection } from '@solana/web3.js';
import { Program } from "@coral-xyz/anchor";
import { PharmaGuardContract } from "../target/types/pharma_guard_contract";


export function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export async function safeAirdrop(address: PublicKey, connection: Connection) {
    // 获取空投前的余额
    const initialBalance = await connection.getBalance(address, "confirmed");
    console.log(`Initial balance of ${address.toString()}: ${initialBalance / LAMPORTS_PER_SOL} SOL`);

    // 检查是否需要空投
    if (initialBalance < LAMPORTS_PER_SOL) {
        console.log("Requesting airdrop...");
        let signature = await connection.requestAirdrop(address, LAMPORTS_PER_SOL);
        
        // 确认交易
        await connection.confirmTransaction(signature);
        console.log("Airdrop transaction confirmed.");

        // 延迟一段时间以确保余额更新
        await delay(2000);

        // 获取空投后的余额
        const finalBalance = await connection.getBalance(address, "confirmed");
        console.log(`Final balance of ${address.toString()}: ${finalBalance / LAMPORTS_PER_SOL} SOL`);

        // 检查余额是否增加
        if (finalBalance > initialBalance) {
            console.log(`Airdrop successful: Received ${(finalBalance - initialBalance) / LAMPORTS_PER_SOL} SOL.`);
        } else {
            console.error(`Airdrop failed: Balance did not change.`);
        }
    } else {
        console.log(`No airdrop needed: Account already has sufficient balance.`);
    }
}


export async function getDrugInfo(program: Program<PharmaGuardContract>, userPharmacyAssociatedAccount: PublicKey, pharmacyIndex: number, drugIndex: number) {
    const userPAA = await program.account.userPharmacyAssociatedAccount.fetch(userPharmacyAssociatedAccount);
    const drug = userPAA.pharmacyInfos[pharmacyIndex].orderHistory[drugIndex];
    return drug;
}
