use {
    crate::pda::PDA,
    anchor_lang::{prelude::*, AnchorDeserialize},
    std::convert::TryFrom,
};

pub const SEED_FEE: &[u8] = b"fee";

/**
 * Fee
 */

#[account]
#[derive(Debug)]
pub struct Fee {
    pub balance: u64,
    pub manager: Pubkey,
}

impl TryFrom<Vec<u8>> for Fee {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Fee::try_deserialize(&mut data.as_slice())
    }
}

impl Fee {
    pub fn pda(manager: Pubkey) -> PDA {
        Pubkey::find_program_address(&[SEED_FEE, manager.as_ref()], &crate::ID)
    }
}

/**
 * FeeAccount
 */

pub trait FeeAccount {
    fn new(&mut self, manager: Pubkey) -> Result<()>;

    fn collect(&mut self, to: &mut Signer) -> Result<()>;
}

impl FeeAccount for Account<'_, Fee> {
    fn new(&mut self, manager: Pubkey) -> Result<()> {
        self.balance = 0;
        self.manager = manager;
        Ok(())
    }

    fn collect(&mut self, to: &mut Signer) -> Result<()> {
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_sub(self.balance)
            .unwrap();
        **to.to_account_info().try_borrow_mut_lamports()? = to
            .to_account_info()
            .lamports()
            .checked_add(self.balance)
            .unwrap();

        self.balance = 0;

        Ok(())
    }
}
