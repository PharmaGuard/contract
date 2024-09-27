
pub mod instructions;
use instructions::*;


use anchor_lang::prelude::*;


declare_id!("7Yh9T122Q7guDzQe2FSYRcKj443AUvNpv3P9PeB7vGCt");

#[program]
pub mod pharma_guard_contract {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    // 创建药品记录
    pub fn create_medication(ctx: Context<CreateMedication>, data: MedicationData) -> Result<()> {
        process_create(ctx, data)
    }

    // // 更新药品记录
    // pub fn update_medication(ctx: Context<UpdateMedication>, data: MedicationData) -> Result<()> {
    //     instructions::update::process_update(ctx, data)
    // }

    // // 删除药品记录
    // pub fn delete_medication(ctx: Context<DeleteMedication>) -> Result<()> {
    //     instructions::delete::process_delete(ctx)
    // }

    // // 读取药品记录
    // pub fn read_medication(ctx: Context<ReadMedication>) -> Result<MedicationData> {
    //     instructions::read::process_read(ctx)
    // }
}