use crate::parse::binary::{Consume, Parse, ParseError, ParsingData};
use crate::wasm::data::{Data, DataMode};
use crate::wasm::indices::MemIdx;
use crate::wasm::instr::Expr;
use crate::wasm::values::Byte;
use crate::wasm::values::U32;

#[derive(Debug, Default)]
pub struct DataSection {
    pub data: Vec<Data>,
}

impl Parse for DataSection {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        Ok(DataSection {
            data: Vec::parse(data)?,
        })
    }
}

impl Parse for Data {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        if data.len() < 1 {
            return Err(ParseError::new(
                data,
                "Can't decode Data. Buffer too small.".to_string(),
            ));
        }
        let d = match data.consume(()) {
            0x00 => {
                let e = Expr::parse(data)?;
                let b: Vec<Byte> = Vec::parse(data)?;
                Data {
                    init: b,
                    mode: DataMode::Active {
                        memory: MemIdx(U32(0)),
                        offset: e,
                    },
                }
            }
            0x01 => {
                let b: Vec<Byte> = Vec::parse(data)?;
                Data {
                    init: b,
                    mode: DataMode::Passive,
                }
            }
            0x02 => {
                let x = MemIdx::parse(data)?;
                let e = Expr::parse(data)?;
                let b: Vec<Byte> = Vec::parse(data)?;
                Data {
                    init: b,
                    mode: DataMode::Active {
                        memory: x,
                        offset: e,
                    },
                }
            }
            x => {
                return Err(ParseError::new(
                    data,
                    format!("Unknown data option: {:X}", x),
                ))
            }
        };
        Ok(d)
    }
}
