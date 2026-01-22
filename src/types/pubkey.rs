use solana_program::{program_error::ProgramError, pubkey::Pubkey};

use crate::traits::{AnchorDeserialize, AnchorSerialize};

impl AnchorSerialize for Pubkey {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.len() < 32 {
            return Err(ProgramError::InvalidAccountData);
        }
        buf[..32].copy_from_slice(self.as_ref());
        Ok(32)
    }
}

impl AnchorDeserialize for Pubkey {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.len() < 32 {
            return Err(ProgramError::InvalidAccountData);
        }

        let arr: [u8; 32] = data[..32]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        let pubkey = Pubkey::new_from_array(arr);

        Ok((pubkey, 32))
    }
}
