mod foo;

use crate::foo::Bar;

//Nested modules
mod food {
    #[derive(Debug)]
    pub struct Cake;
    
}

use food::*;

fn main() {
    let eatable = Cake;
    println!("Hello, world - {:?}", eatable);
    let _ = Bar::init();
}
