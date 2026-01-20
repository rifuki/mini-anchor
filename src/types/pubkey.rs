use solana_program::pubkey::Pubkey;

use crate::traits::{AnchorDeserialize, AnchorSerialize};

impl AnchorSerialize for Pubkey {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.len() < 32 {
            return Err("Buffer too small for Pubkey");
        }
        buf[..32].copy_from_slice(self.as_ref());
        Ok(32)
    }

    fn size() -> usize {
        32
    }
}

impl AnchorDeserialize for Pubkey {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.len() < 32 {
            return Err("Data too short for Pubkey");
        }

        let arr: [u8; 32] = data[..32]
            .try_into()
            .map_err(|_| "Failed to convert to Pubkey")?;

        Ok((Pubkey::new_from_array(arr), 32))
    }

    fn size() -> usize {
        32
    }
}
