#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;



const HEADER: &'static [u8; 4] = b"Obj\0"; //Inlined

//stays at same mem location - singleton
static mut BAZ: u32 = 4; 
static FOO: u8 =9;

const fn salt(a: u32) -> u32 {
    0xDEABDEEF ^ a //xor
}

const CHECKSUM: u32 = salt(23);

const fn read_header(a: &[u8]) -> (u8, u8, u8, u8) {
    (a[0], a[1], a[2], a[3])
}

const FILE_HEADER: (u8, u8, u8, u8) = read_header(include_bytes!("main.rs"));


lazy_static! {
    static ref PATTERNS: HashMap<u32, &'static str> ={
        let mut m = HashMap::new();
        m.insert(0, "zero");
        m.insert(1, "one");
        m.insert(2, "two");
        m
    };
}


fn main() {
    println!("{:?}", HEADER);
    
    unsafe{
        println!("BAZ is {}", BAZ);
        BAZ = 54;
        println!("BAZ is {}", BAZ);
        println!("FOO is {}", FOO);
    }

    println!("{:?}", CHECKSUM);
    println!("{:?}", FILE_HEADER);

    println!("The entry for `0` is \"{}\".", PATTERNS.get(&0).unwrap());

}
