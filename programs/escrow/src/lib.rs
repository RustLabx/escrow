use anchor_lang::prelude::*;

declare_id!("8Rs8uDwWMZjHRUV4GG1bWN3wYide4k1GLb5NywHKnUuM");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
