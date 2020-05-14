enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};

fn main(){
    let list = Cons(1,Box::new(Cons(1,Box::new(Cons(1, Box::new(Nil))))));
}