#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E)
}

fn main() {
    let a = divide(10, 5);
    let b = divide(10, 0);

    println!("a = {:?}, b={:?}", a, b);

    match a{
        Res::Thing(v) => println!("val = {}", v),
        _ => {}
    }

    //statement above if we are interested in only one case to be true
    // can be written with if let as below
    if let Res::Thing(v) = a {
        println!("another val = {}", v);
    }

    let a = divide_std(10, 5);
    let b = divide_std(10, 0);

    
    if let Ok(v) = a {
        println!("std val = {}", v);
    }

}

fn divide(a: i32, b: i32) -> Res<i32, String>{
    if b==0 {
        return Res::Error("Cannot divide by zero".to_string());
    }
    Res::Thing(a/b)
}

fn divide_std(a: i32, b: i32) -> Result<i32, String>{
    if b==0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(a/b)
}