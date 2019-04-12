use std::str;
/*
GENERIC FUNCTIONS
-----------------
cargo build
cd target/debug
nm generics | grep "give"

Now you'll see two functions in compiled object (one for each type)
0000000100001d90 t __ZN8generics7give_me17h0f3a55529ae67438E
0000000100001db0 t __ZN8generics7give_me17h362b4a0b778e38d5E
0000000100001dc0 t __ZN8generics7give_me17h7022f356fd986234E
 */
fn give_me<T>(value: T){
    let _=value;
}

// GENERIC TYPES
struct GenericStruct<T>(T);
#[derive(Debug)]
struct Container<T>{
    item: T
}
enum Transmission<T>{
    Signal(T),
    NoSignal
}

//GENRIC IMPLEMENTATIONS (impl)
// define type before using impl<T> ....
impl<T> Container<T>{
    fn new(item: T) -> Self {
        Container{item}
    }
}
impl Container<u32>{
    fn sum(item: u32) -> Self {
        Container{item}
    }
}

fn main() {
    let a ="generics";
    let b = 1024;
    let c = vec![1,2,3];
    give_me(a);
    give_me(b);
    give_me(c);

    let _s1 = GenericStruct(8);
    let _s2 = GenericStruct("ABC");
    let _s3 = Container{ item:8};
    let _s4 = Container{ item:"ABC"};
    let _s5:Transmission<u8> = Transmission::NoSignal;
    let _s6 = Transmission::Signal(8);
    let _s7 = Transmission::Signal("ABC");
    let s8 = Container::new(8);
    println!("{:?}", s8);
    let s9 = Container::sum(8);
    println!("{:?}", s9);

    //USING GENERICS
    //by providing type
    let _u1:Vec<u32> = Vec::new();

    //by calling a method
    let mut u2 = Vec::new();
    u2.push(8); // was only able to establish type from element

    //use turbofish
    let _u3 = Vec::<u32>::new();

    let num_from_str = str::parse::<u32>("34").unwrap();
    println!("Parsed number {}", num_from_str);
}
