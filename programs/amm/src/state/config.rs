use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Option<Pubkey>,
    pub seed: u64,
    pub fee: u16,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub locked: bool,
    pub config_bump: u8,
    pub lp_bump: u8,
}

impl Config {
    const INIT_SPACE: usize = 8 + std::mem::size_of::<Config>();

    pub fn set_inner(&mut self, inner: Config) -> Result<()> {
        *self = inner;
        Ok(())
    }
}