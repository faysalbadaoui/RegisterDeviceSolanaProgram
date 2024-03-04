use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Device {
    pub authority: Pubkey,
    pub deviceMAC: String,
    pub deviceName: String
}