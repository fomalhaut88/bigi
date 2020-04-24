use std::env;
use std::fs;
use std::io::prelude::*;


fn main() {
    let mut file = fs::File::create("src/constants.rs").unwrap();

    let bigi_bits = match env::var("BIGI_BITS") {
        Err(_why) => "256".to_string(),
        Ok(value) => value
    };

    file.write_fmt(format_args!(
        "pub const BIGI_BITS: usize = {};\n",
        bigi_bits
    )).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=BIGI_BITS");
}
