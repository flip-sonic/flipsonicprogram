use anchor_lang::prelude::*;

declare_id!("5MnVMGkWMYz5NUnrt9So1qwHQRauFa7aqhMfj83mAMBU");

#[program]
pub mod flipsonicprogram {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
