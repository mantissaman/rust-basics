use std::path::Path;
use std::fs::File;
use std::io::Read;

fn infer_types(){
    let _my_result: Result<_, ()> = Ok(55);
    let _my_result = Ok::<_, ()>(55);

    let _my_err: Result<(), f32> = Err(45.3);
    let _my_err = Err::<(), f32>(45.3);

}

fn read_file() {
    let path = Path::new("data.txt");
    //let file = File::open(path);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Error while opeing file {}", err)
    };
    let mut s = String::new();
    let _ = file.read_to_string(&mut s);
    println!("Message: {}", s);
}

fn divide(number: i32, divisor: i32) -> Option<i32> {
    if divisor != 0 {
        Some(number/divisor)
    }
    else{
        None
    }
}

fn to_message(number: Option<i32>) -> String {
    number.map(|n| format!("{} is a number!", n))
    .unwrap_or("None!".to_string())
}

fn comb(){
    let some_number = Some(9);

    let another_number = some_number
    .map(|n| n-1) // Some(8)
    .map (|n| n*n)  //Some(64)
    .and_then(|n| divide(n, 4)); //pass a function that returns option type

    println!("{}", to_message(another_number));

    let final_number = another_number
    .and_then(|n| divide(n, 0));
    println!("{}", to_message(final_number));
}

fn main() {
    infer_types();
    read_file();
    comb();
}
