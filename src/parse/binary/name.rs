use crate::parse::Parse;

#[derive(Debug)]
pub struct Name {
    name: String,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Inocrrect name format")]
    IncorrectFormat,
    #[error("UTF8 error")]
    UTF8Error(#[from] std::str::Utf8Error),
}

impl<'a> Parse<'a> for Name {
    type Error = ParseError;
    fn parse(data: &mut &'a [u8]) -> Result<Self, Self::Error> {
        let mut name: Vec<u8>;
        name.parse_into(&mut data)
            .map_err(|_| ParseError::IncorrectFormat)?;

        Ok(Name {
            name: std::str::from_utf8(&name)?.into(),
        })
    }
}
