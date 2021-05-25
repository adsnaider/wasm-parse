use super::{Consume, Parse, ParseError, ParsingData};

const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];

pub enum Version {
    V1_0_0_0,
}

pub struct Preamble {
    pub version: Version,
}

impl Parse for Preamble {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let magic = data.consume(4);
        if *magic != MAGIC {
            return Err(ParseError::new(
                data,
                format!("Invalid magic! Expected {:X?}, got {:X?}", &MAGIC, &magic),
            ));
        }
        let version = data.consume(4);
        return match *version {
            [1, 0, 0, 0] => Ok(Preamble {
                version: Version::V1_0_0_0,
            }),
            _ => Err(ParseError::new(
                data,
                format!("Unknown version {:X?}", version),
            )),
        };
    }
}
