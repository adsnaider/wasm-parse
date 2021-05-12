pub mod preamble;
pub mod section;
pub mod vector;

pub trait Parse<'a>: Sized {
    type Error: std::error::Error + std::fmt::Display + std::fmt::Debug + 'static;
    fn parse(data: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error>;
}

pub struct BinaryModule<'a> {
    header: preamble::Preamble,
    sections: Vec<section::Section<'a>>,
}
