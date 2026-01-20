use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

use std::marker::PhantomData;

pub struct Program<'info, T> {
    pub info: &'info AccountInfo<'info>,
    _marker: PhantomData<T>,
}

impl<'info, T> Program<'info, T> {
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
