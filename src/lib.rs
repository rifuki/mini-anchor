use std::marker::PhantomData;

use solana_program::pubkey::Pubkey;
pub use solana_program::{self, account_info::AccountInfo};

pub use mini_anchor_macro::account;
pub use mini_anchor_macro::declare_id;

pub trait AnchorSerialize {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str>;
    // Unused for variable-length types like String
    fn size() -> usize;
}

pub trait AnchorDeserialize: Sized {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str>;
    // Unused for variable-length types like String
    fn size() -> usize;
}

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

impl AnchorSerialize for String {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        let str_bytes = self.as_bytes();
        let total_size = 4 + str_bytes.len();

        // Need: 4 bytes for length + bytes for string
        if buf.len() < total_size {
            return Err("Buffer too small for String");
        }

        // Write length prefix (u32 little-endian)
        let str_len_u32 = str_bytes.len() as u32;
        buf[..4].copy_from_slice(&str_len_u32.to_le_bytes());
        // Write string bytes
        buf[4..total_size].copy_from_slice(str_bytes);

        Ok(total_size)
    }
    fn size() -> usize {
        0
    }
}

impl AnchorDeserialize for String {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.len() < 4 {
            return Err("Data too short for String length");
        }

        let u32_len_arr =
            u32::from_le_bytes(data[..4].try_into().map_err(|_| "Invalid u32 length")?) as usize;
        let total_size = 4 + u32_len_arr;

        if data.len() < total_size {
            return Err("Data too short for String content");
        }

        let string_bytes = &data[4..total_size];

        let string_value =
            String::from_utf8(string_bytes.to_vec()).map_err(|_| "Invalid UTF-8 string")?;

        Ok((string_value, total_size))
    }
    fn size() -> usize {
        0
    }
}

impl<T: AnchorSerialize> AnchorSerialize for Option<T> {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.is_empty() {
            return Err("Buffer too small for Option");
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

    fn size() -> usize {
        0 // Dynamic size
    }
}

impl<T: AnchorDeserialize> AnchorDeserialize for Option<T> {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.is_empty() {
            return Err("Data too short for Option");
        }

        match data[0] {
            0 => Ok((None, 1)),
            1 => {
                let (value, read) = T::deserialize(&data[1..])?;
                Ok((Some(value), 1 + read))
            }
            _ => Err("Invalid Option discriminant"),
        }
    }

    fn size() -> usize {
        0 // Dynamic size
    }
}

impl<T: AnchorSerialize> AnchorSerialize for Vec<T> {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if buf.len() < 4 {
            return Err("Buffer too small for Vec length");
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

    fn size() -> usize {
        0 // Dynamic size
    }
}

impl<T: AnchorDeserialize> AnchorDeserialize for Vec<T> {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), &'static str> {
        if data.len() < 4 {
            return Err("Data too short for Vec length");
        }

        let len =
            u32::from_le_bytes(data[..4].try_into().map_err(|_| "Invalid u32 length")?) as usize;

        let mut offset = 4;
        let mut result = Vec::with_capacity(len);

        for _ in 0..len {
            let (item, read) = T::deserialize(&data[offset..])?;
            result.push(item);
            offset += read;
        }

        Ok((result, offset))
    }

    fn size() -> usize {
        0 // Dynamic size
    }
}

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
impl<'info, T: AnchorDeserialize> Account<'info, T> {
    pub fn data(&self) -> Result<T, &'static str> {
        let data = self.info.data.borrow();
        let (value, _) = T::deserialize(&data)?;
        Ok(value)
    }
}

pub struct Signer<'info> {
    pub info: &'info AccountInfo<'info>,
}

impl<'info> Signer<'info> {
    pub fn new(info: &'info AccountInfo<'info>) -> Result<Self, &'static str> {
        if !info.is_signer {
            return Err("Account must be a signer");
        }
        Ok(Self { info })
    }

    pub fn key(&self) -> &Pubkey {
        self.info.key
    }
}

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

pub mod prelude {
    pub use crate::AnchorDeserialize;
    pub use crate::AnchorSerialize;
    pub use crate::account;
    pub use crate::declare_id;
    pub use crate::solana_program;
}
