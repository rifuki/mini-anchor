use crate::traits::AnchorDeserialize;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

use std::marker::PhantomData;

pub struct Account<'info, T> {
    pub info: &'info AccountInfo<'info>,
    _marker: PhantomData<T>,
}

impl<'info, T> Account<'info, T> {
    pub fn new(info: &'info AccountInfo<'info>) -> Self {
        Self {
            info,
            _marker: PhantomData,
        }
    }

    pub fn key(&self) -> &Pubkey {
        self.info.key
    }
}

// Deserialize account data into T
impl<T: AnchorDeserialize> Account<'_, T> {
    pub fn data(&self) -> Result<T, &'static str> {
        let data = self.info.data.borrow();
        let (value, _) = T::deserialize(&data)?;
        Ok(value)
    }
}
