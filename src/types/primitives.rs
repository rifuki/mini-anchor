use crate::traits::{AnchorDeserialize, AnchorSerialize};

// ============ i8 ============
impl AnchorSerialize for i8 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.is_empty() {
            return Err("Buffer too small for i8");
        }
        // buf[0] = *self as u8; OR
        buf[0] = self.to_le_bytes()[0];
        Ok(1)
    }

    fn size() -> usize {
        1
    }
}

impl AnchorDeserialize for i8 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.is_empty() {
            return Err("Data too short for i8");
        }
        Ok((i8::from_le_bytes([data[0]]), 1))
    }

    fn size() -> usize {
        1
    }
}

// ============ u8 ============
impl AnchorSerialize for u8 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.is_empty() {
            return Err("Buffer too small for u8");
        }
        buf[0] = *self;
        Ok(1)
    }
    fn size() -> usize {
        1
    }
}

impl AnchorDeserialize for u8 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.is_empty() {
            return Err("Data too short for u8");
        }
        Ok((data[0], 1))
    }
    fn size() -> usize {
        1
    }
}

// =========== i16 ===========
impl AnchorSerialize for i16 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.len() < 2 {
            return Err("Buffer too small for i16");
        }
        buf[..2].copy_from_slice(&self.to_le_bytes());
        Ok(2)
    }

    fn size() -> usize {
        2
    }
}

impl AnchorDeserialize for i16 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.len() < 2 {
            return Err("Data too short for i16");
        }

        let arr: [u8; 2] = data[..2]
            .try_into()
            .map_err(|_| "Failed to convert to i16")?;

        Ok((i16::from_le_bytes(arr), 2))
    }

    fn size() -> usize {
        2
    }
}

// =========== u16 ===========
impl AnchorSerialize for u16 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.len() < 2 {
            return Err("Buffer too small for u16");
        }
        buf[..2].copy_from_slice(&self.to_le_bytes());
        Ok(2)
    }

    fn size() -> usize {
        2
    }
}

impl AnchorDeserialize for u16 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.len() < 2 {
            return Err("Data too short for u16");
        }
        let arr: [u8; 2] = data[..2]
            .try_into()
            .map_err(|_| "Failed to convert to u16")?;
        Ok((u16::from_le_bytes(arr), 2))
    }

    fn size() -> usize {
        2
    }
}

// =========== i32 ===========
impl AnchorSerialize for i32 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.len() < 4 {
            return Err("Buffer too small for i32");
        }
        buf[..4].copy_from_slice(&self.to_le_bytes());
        Ok(4)
    }

    fn size() -> usize {
        4
    }
}

impl AnchorDeserialize for i32 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.len() < 4 {
            return Err("Data too short for i32");
        }
        let arr: [u8; 4] = data[..4]
            .try_into()
            .map_err(|_| "Failed to convert to i32")?;
        Ok((i32::from_le_bytes(arr), 4))
    }

    fn size() -> usize {
        4
    }
}

// ============ u32 ============
impl AnchorSerialize for u32 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.len() < 4 {
            return Err("Buffer too small for u32");
        }
        buf[..4].copy_from_slice(&self.to_le_bytes());
        Ok(4)
    }

    fn size() -> usize {
        4
    }
}

impl AnchorDeserialize for u32 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.len() < 4 {
            return Err("Data too short for u32");
        }
        let arr: [u8; 4] = data[..4]
            .try_into()
            .map_err(|_| "Failed to convert to u32")?;
        Ok((u32::from_le_bytes(arr), 4))
    }

    fn size() -> usize {
        4
    }
}

// =========== u64 ===========
impl AnchorSerialize for i64 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.len() < 8 {
            return Err("Buffer too small for i64");
        }
        buf[..8].copy_from_slice(&self.to_le_bytes());
        Ok(8)
    }
    fn size() -> usize {
        8
    }
}

impl AnchorDeserialize for i64 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.len() < 8 {
            return Err("Data too short for i64");
        }
        let arr: [u8; 8] = data[..8]
            .try_into()
            .map_err(|_| "Failed to convert to i64")?;
        Ok((i64::from_le_bytes(arr), 8))
    }
    fn size() -> usize {
        8
    }
}

// =========== u64 ===========
impl AnchorSerialize for u64 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.len() < 8 {
            return Err("Buffer too small for u64");
        }
        buf[..8].copy_from_slice(&self.to_le_bytes());
        Ok(8)
    }
    fn size() -> usize {
        8
    }
}

impl AnchorDeserialize for u64 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.len() < 8 {
            return Err("Data too short for u64");
        }
        let arr: [u8; 8] = data[..8]
            .try_into()
            .map_err(|_| "Failed to convert to u64")?;
        Ok((u64::from_le_bytes(arr), 8))
    }
    fn size() -> usize {
        8
    }
}

// ============ i128 ============
impl AnchorSerialize for i128 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.len() < 16 {
            return Err("Buffer too small for i128");
        }
        buf[..16].copy_from_slice(&self.to_le_bytes());
        Ok(16)
    }
    fn size() -> usize {
        16
    }
}

impl AnchorDeserialize for i128 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.len() < 16 {
            return Err("Data too short for i128");
        }
        let arr: [u8; 16] = data[..16]
            .try_into()
            .map_err(|_| "Failed to convert to i128")?;
        Ok((i128::from_le_bytes(arr), 16))
    }

    fn size() -> usize {
        16
    }
}

// ============ u128 ============
impl AnchorSerialize for u128 {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.len() < 16 {
            return Err("Buffer too small for u128");
        }
        buf[..16].copy_from_slice(&self.to_le_bytes());
        Ok(16)
    }

    fn size() -> usize {
        16
    }
}

impl AnchorDeserialize for u128 {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.len() < 16 {
            return Err("Data too short for u128");
        }
        let arr: [u8; 16] = data[..16]
            .try_into()
            .map_err(|_| "Failed to convert to u128")?;
        Ok((u128::from_le_bytes(arr), 16))
    }

    fn size() -> usize {
        16
    }
}

// ============ bool ============
impl AnchorSerialize for bool {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.is_empty() {
            return Err("Buffer too small for bool");
        }
        buf[0] = if *self { 1 } else { 0 };
        Ok(1)
    }
    fn size() -> usize {
        1
    }
}

impl AnchorDeserialize for bool {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.is_empty() {
            return Err("Data too short for bool");
        };
        Ok((data[0] != 0, 1))
    }

    fn size() -> usize {
        1
    }
}
