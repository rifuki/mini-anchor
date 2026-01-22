use solana_program::program_error::ProgramError;

use crate::traits::{AnchorDeserialize, AnchorSerialize};

impl<T: AnchorSerialize> AnchorSerialize for Option<T> {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.is_empty() {
            return Err(ProgramError::InvalidAccountData);
        }

        match self {
            None => {
                buf[0] = 0;
                Ok(1)
            }
            Some(value) => {
                buf[0] = 1;
                let written = value.serialize(&mut buf[1..])?;
                Ok(1 + written)
            }
        }
    }
}

impl<T: AnchorDeserialize> AnchorDeserialize for Option<T> {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.is_empty() {
            return Err(ProgramError::InvalidAccountData);
        }

        match data[0] {
            0 => Ok((None, 1)),
            1 => {
                let (value, read) = T::deserialize(&data[1..])?;
                Ok((Some(value), 1 + read))
            }
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
}
