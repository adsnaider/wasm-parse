//use wasm_parse::wasm::Module;
use std::fs::File;
use std::io::Read;
use wasm_parse::parse::Parse;
use wasm_parse::wasm::module::Module;

fn main() {
    let mut wasm = File::open("../target/wasm32-unknown-unknown/debug/example.wasm")
        .expect("Can't open wasm file");
    let mut data = Vec::new();
    wasm.read_to_end(&mut data).expect("Couldn't read file");
    let wasm: Module = Module::parse(&mut data.as_slice()).expect("Coudln't decode module");
    println!("{:?}", wasm);
}
