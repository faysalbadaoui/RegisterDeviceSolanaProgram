use anchor_lang::prelude::*;

pub mod constant;
pub mod device; //Add device object into the file
use crate::{constant::*, device::*}; //import the class of device to be able to use it

declare_id!("duyihVCWQB2azfZDeXoxKeZeQ1v9HVfWaTDDHdYhBau");

#[program]
pub mod register_devices {
    //Initialize a device
    use super::*;

    pub fn initialize_device(
        ctx: Context<InitializeDevice>,
        _deviceMAC: String,
        _deviceName: String,
    ) -> Result<()> {
        //definition of the function with the input struct expected
        //Initialize the device into the blockchain
        let device_object = &mut ctx.accounts.device_object;
        device_object.authority = ctx.accounts.authority.key();
        device_object.deviceMAC = _deviceMAC;
        device_object.deviceName = _deviceName;
        Ok(())
    }

    pub fn delete_device(ctx: Context<DeleteDevice>) -> Result<()> {
        let device_account = &ctx.accounts.device_object;

        // Ensure that the signer is the owner of the Device account
        if device_account.authority == ctx.accounts.authority.key() {
            ctx.accounts
                .device_object
                .close(ctx.accounts.authority.to_account_info())?;
        }
        Ok(())
    }

}
// Define the context for deleting a Device account
#[derive(Accounts)]
#[instruction()]
pub struct DeleteDevice<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // Signer must be the owner of the Device account

    #[account(mut, close = authority)]
    pub device_object: Box<Account<'info, Device>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeDevice<'info> {
    #[account(mut)] //to make the authority mutable
    pub authority: Signer<'info>, //The wallet that is in charge of the account saving the data of that device

    #[account( 
        init,
        seeds = [DEVICE_TAG,authority.key().as_ref()], //The generation of random DID using a tag and the signer 
        bump, //bump is usefult to not have duplicates, if it exists already with a did will add a 1 to the seeds
        payer = authority,
        space = 8 + std::mem::size_of::<Device>(),
    )]
    pub device_object: Box<Account<'info, Device>>,

    pub system_program: Program<'info, System>,
}
