use super::Parse;
use nano_leb128::ULEB128;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VecParseError<E: std::fmt::Debug + std::fmt::Display + std::error::Error + 'static> {
    #[error("Can't decode vector")]
    VectorMalformed,
    #[error(transparent)]
    DataError(#[from] E),
}

impl<'a, T> Parse<'a> for Vec<T>
where
    T: Parse<'a>,
    T::Error: std::fmt::Debug + std::fmt::Display + std::error::Error,
{
    type Error = VecParseError<T::Error>;
    fn parse(data: &'a [u8]) -> Result<(Self, &'a [u8]), Self::Error> {
        let (value, len) =
            ULEB128::read_from(&data.get(0..).ok_or(VecParseError::VectorMalformed)?)
                .or(Err(VecParseError::VectorMalformed))?;
        let mut result = Vec::new();
        let value = u64::from(value) as usize;
        result.reserve_exact(value);
        let data = &data[len..];
        for i in 0..value {
            let (t, data) = T::parse(data)?;
            result.push(t);
        }
        Ok((result, data))
    }
}
