use solana_program::program_error::ProgramError;

use crate::traits::{AnchorDeserialize, AnchorSerialize};

// ============ i8 ============
impl AnchorSerialize for i8 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.is_empty() {
            return Err(ProgramError::InvalidAccountData);
        }
        // buf[0] = *self as u8; OR
        buf[0] = self.to_le_bytes()[0];
        Ok(1)
    }
}

impl AnchorDeserialize for i8 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.is_empty() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok((i8::from_le_bytes([data[0]]), 1))
    }
}

// ============ u8 ============
impl AnchorSerialize for u8 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.is_empty() {
            return Err(ProgramError::InvalidAccountData);
        }
        buf[0] = *self;
        Ok(1)
    }
}

impl AnchorDeserialize for u8 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.is_empty() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok((data[0], 1))
    }
}

// =========== i16 ===========
impl AnchorSerialize for i16 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.len() < 2 {
            return Err(ProgramError::InvalidAccountData);
        }
        buf[..2].copy_from_slice(&self.to_le_bytes());
        Ok(2)
    }
}

impl AnchorDeserialize for i16 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.len() < 2 {
            return Err(ProgramError::InvalidAccountData);
        }
        let arr: [u8; 2] = data[..2]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        Ok((i16::from_le_bytes(arr), 2))
    }
}

// =========== u16 ===========
impl AnchorSerialize for u16 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.len() < 2 {
            return Err(ProgramError::InvalidAccountData);
        }
        buf[..2].copy_from_slice(&self.to_le_bytes());
        Ok(2)
    }
}

impl AnchorDeserialize for u16 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.len() < 2 {
            return Err(ProgramError::InvalidAccountData);
        }
        let arr: [u8; 2] = data[..2]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok((u16::from_le_bytes(arr), 2))
    }
}

// =========== i32 ===========
impl AnchorSerialize for i32 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.len() < 4 {
            return Err(ProgramError::InvalidAccountData);
        }
        buf[..4].copy_from_slice(&self.to_le_bytes());
        Ok(4)
    }
}

impl AnchorDeserialize for i32 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.len() < 4 {
            return Err(ProgramError::InvalidAccountData);
        }
        let arr: [u8; 4] = data[..4]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        Ok((i32::from_le_bytes(arr), 4))
    }
}

// ============ u32 ============
impl AnchorSerialize for u32 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.len() < 4 {
            return Err(ProgramError::InvalidAccountData);
        }
        buf[..4].copy_from_slice(&self.to_le_bytes());
        Ok(4)
    }
}

impl AnchorDeserialize for u32 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.len() < 4 {
            return Err(ProgramError::InvalidAccountData);
        }
        let arr: [u8; 4] = data[..4]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        Ok((u32::from_le_bytes(arr), 4))
    }
}

// =========== u64 ===========
impl AnchorSerialize for i64 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.len() < 8 {
            return Err(ProgramError::InvalidAccountData);
        }
        buf[..8].copy_from_slice(&self.to_le_bytes());
        Ok(8)
    }
}

impl AnchorDeserialize for i64 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.len() < 8 {
            return Err(ProgramError::InvalidAccountData);
        }
        let arr: [u8; 8] = data[..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        Ok((i64::from_le_bytes(arr), 8))
    }
}

// =========== u64 ===========
impl AnchorSerialize for u64 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.len() < 8 {
            return Err(ProgramError::InvalidAccountData);
        }
        buf[..8].copy_from_slice(&self.to_le_bytes());
        Ok(8)
    }
}

impl AnchorDeserialize for u64 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.len() < 8 {
            return Err(ProgramError::InvalidAccountData);
        }
        let arr: [u8; 8] = data[..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        Ok((u64::from_le_bytes(arr), 8))
    }
}

// ============ i128 ============
impl AnchorSerialize for i128 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.len() < 16 {
            return Err(ProgramError::InvalidAccountData);
        }
        buf[..16].copy_from_slice(&self.to_le_bytes());
        Ok(16)
    }
}

impl AnchorDeserialize for i128 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.len() < 16 {
            return Err(ProgramError::InvalidAccountData);
        }
        let arr: [u8; 16] = data[..16]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        Ok((i128::from_le_bytes(arr), 16))
    }
}

// ============ u128 ============
impl AnchorSerialize for u128 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.len() < 16 {
            return Err(ProgramError::InvalidAccountData);
        }
        buf[..16].copy_from_slice(&self.to_le_bytes());
        Ok(16)
    }
}

impl AnchorDeserialize for u128 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.len() < 16 {
            return Err(ProgramError::InvalidAccountData);
        }
        let arr: [u8; 16] = data[..16]
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?;
        Ok((u128::from_le_bytes(arr), 16))
    }
}

// ============ bool ============
impl AnchorSerialize for bool {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError> {
        if buf.is_empty() {
            return Err(ProgramError::InvalidAccountData);
        }
        buf[0] = if *self { 1 } else { 0 };
        Ok(1)
    }
}

impl AnchorDeserialize for bool {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError> {
        if data.is_empty() {
            return Err(ProgramError::InvalidAccountData);
        };
        Ok((data[0] != 0, 1))
    }
}
