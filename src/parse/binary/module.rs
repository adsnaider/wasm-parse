use super::preamble;
use super::sections;
use super::Parse;
use super::ParseError;

pub struct BinaryModule {
    pub header: preamble::Preamble,
    pub sections: Vec<sections::Section>,
}

impl Parse for BinaryModule {
    fn parse(data: &[u8]) -> Result<(Self, usize), ParseError> {
        let mut length = 0;
        let (header, len) = preamble::Preamble::parse(&data[length..])?;
        length += len;
        let mut sections = Vec::new();
        while !data.is_empty() {
            let (section, len) = sections::Section::parse(&data[length..])?;
            length += len;
            sections.push(section);
        }

        Ok((BinaryModule { header, sections }, length))
    }
}
