use std::thread;
use std::panic;

fn alice() -> thread::JoinHandle<()> {
    thread::spawn(move || {
        bob();
    })
}

fn bob(){
    malice();
}

fn malice() {
    panic!("malice is panicking");
}

fn dodo() {
    panic!("dodo is panicking");
}

fn main() {
    let child = alice();
    let _= child.join();
    panic::catch_unwind(|| { 
        dodo();
    }).ok();
    println!("This is no reahcable");
}
