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
