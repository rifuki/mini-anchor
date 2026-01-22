use solana_program::program_error::ProgramError;

use crate::traits::{AnchorDeserialize, AnchorSerialize};

impl AnchorSerialize for String {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        let str_bytes = self.as_bytes();
        let total_size = 4 + str_bytes.len();

        // Need: 4 bytes for length + bytes for string
        if buf.len() < total_size {
            return Err(ProgramError::InvalidAccountData);
        }

        // Write length prefix (u32 little-endian)
        let str_len_u32 = str_bytes.len() as u32;
        buf[..4].copy_from_slice(&str_len_u32.to_le_bytes());
        // Write string bytes
        buf[4..total_size].copy_from_slice(str_bytes);

        Ok(total_size)
    }
}

impl AnchorDeserialize for String {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.len() < 4 {
            return Err(ProgramError::InvalidAccountData);
        }

        let u32_len_arr = u32::from_le_bytes(
            data[..4]
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        ) as usize;

        // Total size = 4 bytes for length + string bytes
        let total_size = 4 + u32_len_arr;
        if data.len() < total_size {
            return Err(ProgramError::InvalidAccountData);
        }

        let string_bytes = &data[4..total_size];
        let string_value = String::from_utf8(string_bytes.to_vec())
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok((string_value, total_size))
    }
}
