use std::thread;
use std::sync::mpsc::{channel, sync_channel};


fn sync_channel_comm(){
    let (tx, rx) = sync_channel(1);
    let tx_clone = tx.clone();
    let _ = tx.send(0);

    thread::spawn(move || {
        let _ = tx.send(1);
    });
    thread::spawn(move || {
        let _ = tx_clone.send(2);
    });
    println!("Received {}", rx.recv().unwrap());
    println!("Received {}", rx.recv().unwrap());
    println!("Received {}", rx.recv().unwrap());
    println!("Received {:?}", rx.recv());

}

fn async_channel(){
    let (tx, rx) = channel();
    let join_handle =thread::spawn(move || {
        while let Ok(n) = rx.recv() {
            println!("Received {}", n);
        }
    });

    for i in 0 ..10 {
        tx.send(i).unwrap();
    }
    join_handle.join().unwrap();

}

fn main() {
    //async_channel();
    sync_channel_comm();
}
