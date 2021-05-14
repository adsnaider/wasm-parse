use crate::parse::binary::name::Name;

#[derive(Debug)]
pub struct CustomSection<'a> {
    name: Name,
    data: &'a [u8],
}
