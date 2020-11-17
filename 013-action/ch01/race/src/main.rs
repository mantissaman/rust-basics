use std::thread;

fn main() {
    let mut data = 100;

    thread::spawn(move || {
        data = 500;
    });
    thread::spawn( move || {
        data = 1000;
    });

    println!("{}", data);
}
