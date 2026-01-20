mod accounts;
mod traits;
mod types;

pub use solana_program::{self, account_info::AccountInfo};

pub use mini_anchor_macro::{account, declare_id};

pub use accounts::{Account, Program, Signer};
pub use traits::{AnchorDeserialize, AnchorSerialize};

pub mod prelude {
    pub use crate::AnchorDeserialize;
    pub use crate::AnchorSerialize;
    pub use crate::account;
    pub use crate::declare_id;
    pub use crate::solana_program;
}
