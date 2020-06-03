
fn apis(){
    let mut empty_string = String::new();
    let mut empty_string_capacity = String::with_capacity(50);
    let string_from_bytestring= String::from_utf8(vec![82,85, 83, 84])
    .expect("Creating from bytestring failed");

    println!("Length of empty stirng is {}", empty_string.len());
    println!("Length of empty string with capacity is {}", empty_string_capacity.len());
    println!("Length of string_from_bytestring is {}", string_from_bytestring.len());
    println!("bytestring is {}", string_from_bytestring);
    empty_string.push('1');
    empty_string_capacity.push_str("12345");
    println!("Length of empty stirng is {}", empty_string.len());
    println!("Length of empty string with capacity is {}", empty_string_capacity.len());   
}


fn strings_construct(){
    
    let _a: String = "Hello".to_string();
    let _b = String::from("Hello");
    let _c = "World".to_owned();
    let _d= _c.clone(); //expensive - avoid

}

fn get_literal() -> &'static str {
    "from function"
}

fn take_slice()  {
    let mystr= String::from("Strings are cool");
    let first_three= &mystr[0..3];
    println!("{:?}", first_three);
}

fn iter_string(){
    let hello= String::from("Hello");
    for c in hello.chars() {
        println!("{}", c);
    }
}

fn say_hello(to_whom: &str){
    println!("Hey {} !", to_whom);
}

fn str_fn(){
    let string_slice: &'static str="you" ;
    let string: String = string_slice.into();
    say_hello(string_slice);
    say_hello(&string);
}


fn main() {
    println!("Hello, world!");
    strings_construct();
    apis();

    let mystr ="This is borrowed";
    let from_fun = get_literal();
    println!("{} {}",mystr, from_fun);
    take_slice();
    iter_string();
    str_fn();

    let foo = "Foo";
    let bar = "Bar";

    let baz = foo.to_string() + bar;
    println!("{}",baz);
}
