use super::Parse;
use std::convert::TryInto;
use thiserror::Error;

const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];

pub enum Version {
    V1_0_0_0,
}

pub struct Preamble {
    pub version: Version,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Header not valid.")]
    IncorrectHeaderFormat,
    #[error("Header magic not valid: (expected {expected:X?}, found {found:X?})")]
    InvalidMagic { expected: [u8; 4], found: [u8; 4] },
    #[error("Unsupported WASM version ({version:?})")]
    UnsupportedVersion { version: [u8; 4] },
}

impl<'a> Parse<'a> for Preamble {
    type Error = ParseError;
    fn parse(data: &mut &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 8 {
            return Err(ParseError::IncorrectHeaderFormat);
        }
        let magic = &data[0..4];
        if magic != &MAGIC {
            return Err(ParseError::InvalidMagic {
                expected: MAGIC,
                found: magic.try_into().unwrap(),
            });
        }
        let version = &data[4..8];
        *data = &data[8..];
        return match version {
            &[1, 0, 0, 0] => Ok(Preamble {
                version: Version::V1_0_0_0,
            }),
            _ => Err(ParseError::UnsupportedVersion {
                version: version.try_into().unwrap(),
            }),
        };
    }
}
