pub enum JobOpcode {
    ForceUpdate,
    DecryptNSignature,
    UnknownOpcode,
}

impl From<u8> for JobOpcode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::ForceUpdate,
            0x01 => Self::DecryptNSignature,
            _ => Self::UnknownOpcode,
        }
    }
}
