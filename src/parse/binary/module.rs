use super::preamble;
use super::sections;
use super::{Parse, ParseError, ParsingData};

pub struct BinaryModule {
    pub header: preamble::Preamble,
    pub sections: Vec<sections::Section>,
}

impl Parse for BinaryModule {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let header =
            preamble::Preamble::parse(data).map_err(|err| err.extend("Can't parse header"))?;
        let mut sections = Vec::new();
        while !data.is_empty() {
            let section = sections::Section::parse(data)
                .map_err(|err| err.extend("Couldn't parse section"))?;
            sections.push(section);
        }

        Ok(BinaryModule { header, sections })
    }
}
