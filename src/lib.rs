#![feature(const_generics)]

pub mod parse;
pub mod wasm;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
