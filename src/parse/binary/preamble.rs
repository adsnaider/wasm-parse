use super::Parse;
use super::ParseError;

const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];

pub enum Version {
    V1_0_0_0,
}

pub struct Preamble {
    pub version: Version,
}

impl Parse for Preamble {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let data = &data[..8];
        if data.len() != 8 {
            return Err(ParseError::new(format!(
                "Header too small. Should be at least 8 bytes. Got {}",
                data.len()
            )));
        }
        let magic = &data[0..4];
        if magic != &MAGIC {
            return Err(ParseError::new(format!(
                "Invalid magic! Expected {:X?}, got {:X?}",
                &MAGIC, &magic
            )));
        }
        let version = &data[4..8];
        return match version {
            &[1, 0, 0, 0] => Ok((
                Preamble {
                    version: Version::V1_0_0_0,
                },
                8,
            )),
            _ => Err(ParseError::new(format!("Unknown version {:X?}", version))),
        };
    }
}
