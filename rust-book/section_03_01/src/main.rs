fn shadow_var(){
    let y = 5;
    println!("The value of y is: {}", y);
    let y = y +5;
    println!("The unshadowed value of y  is: {}", y);

    //Use it to reuse the varible - keep context
    //change datatype

    let spaces ="     ";
    let spaces = spaces.len();
    println!("Spaces: {}", spaces);
}

fn main() {
    let mut x =5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The mutated value of x is: {}", x);
    shadow_var();
}
