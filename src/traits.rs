use solana_program::program_error::ProgramError;

pub trait AnchorSerialize {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, ProgramError>;
}

pub trait AnchorDeserialize: Sized {
    fn deserialize(data: &[u8]) -> Result<(Self, usize), ProgramError>;
}
