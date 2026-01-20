use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

pub struct Signer<'info> {
    pub info: &'info AccountInfo<'info>,
}

impl<'info> Signer<'info> {
    pub fn new(info: &'info AccountInfo<'info>) -> Result<Self, &'static str> {
        if !info.is_signer {
            return Err("Account must be a signer");
        }
        Ok(Self { info })
    }

    pub fn key(&self) -> &Pubkey {
        self.info.key
    }
}
