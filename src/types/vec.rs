use solana_program::program_error::ProgramError;

use crate::traits::{AnchorDeserialize, AnchorSerialize};

impl<T: AnchorSerialize> AnchorSerialize for Vec<T> {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.len() < 4 {
            return Err(ProgramError::InvalidAccountData);
        }

        // Write length prefix (u32 little-endian)
        let len = self.len() as u32;
        buf[..4].copy_from_slice(&len.to_le_bytes());

        let mut offset = 4;
        for item in self.iter() {
            let written = item.serialize(&mut buf[offset..])?;
            offset += written;
        }

        Ok(offset)
    }
}

impl<T: AnchorDeserialize> AnchorDeserialize for Vec<T> {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.len() < 4 {
            return Err(ProgramError::InvalidAccountData);
        }

        let len = u32::from_le_bytes(
            data[..4]
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        ) as usize;

        let mut offset = 4;
        let mut result = Vec::with_capacity(len);

        for _ in 0..len {
            let (item, read) = T::deserialize(&data[offset..])?;
            result.push(item);
            offset += read;
        }

        Ok((result, offset))
    }
}
