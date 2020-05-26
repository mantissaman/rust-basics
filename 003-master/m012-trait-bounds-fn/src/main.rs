
 use std::fmt::Debug;

trait Eatable{
    fn eat(&self);
}

#[derive(Debug)]
struct Food<T>(T);

#[derive(Debug)]
struct Apple;

impl<T> Eatable for Food<T> where T: Debug{
    fn eat(&self){
        println!("Eating {:?}", self);
    }
}

// fn eat<T: Eatable> (val: T) {
//     val.eat();
// }

fn eat(val: impl Eatable) {
    val.eat();
}

fn lazy_add(a: u32, b:u32) -> impl Fn() -> u32 {
    move || a + b
}



fn main() {
    let apple = Food(Apple);
    eat(apple);

    let add_later = lazy_add(1024, 2024);
    println!("{}", add_later());
}
