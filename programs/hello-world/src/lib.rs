use anchor_lang::prelude::*;

declare_id!("9CwELQhKkRYjaSRu41DboDj64Huv8ndaQoTzzFDUpVRR");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
