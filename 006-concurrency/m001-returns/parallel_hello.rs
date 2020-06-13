use std::thread;

fn main(){
    //thread spwaning will take more time - no free lunches
    thread::spawn (|| {
       println!("Greetings, Humans"); 
    }).join().unwrap();
}

//rustc -C opt-level=3 parallel_hello.rs
//time for i in {1..100}; do ./parallel_hello > /dev/null; done