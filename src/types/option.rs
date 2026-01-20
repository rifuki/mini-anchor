use crate::traits::{AnchorDeserialize, AnchorSerialize};

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
