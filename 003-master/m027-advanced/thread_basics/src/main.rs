use std::thread;
use std::thread::Builder;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

fn thread_basics() {
    let child = thread::spawn(|| {
        println!("Thread!");
        "Much concurrent, such wow!".to_string()
    });

    print!("Hello");
    let value = child.join().expect("Failed joining chil thread");
    println!("{}", value);
}

fn thread_customize(){
    let my_thread =Builder::new().name("Worker Thread".to_string()). stack_size(1024 * 4);
    let handle = my_thread.spawn(|| {
        panic!("Oops!");
    });
    let child_status = handle.unwrap().join();
    println!("Child satus: {:?}", child_status);
}

fn thread_data(){
    let  nums = Arc::new(vec![0,1,2,3,4]);
    let mut children = vec![];
    for n in 0..5{
        let num = Arc::clone(&nums);
        let c = thread::spawn( move || {
            println!("{}", num[n]);
        });
        children.push(c);
    }

    for c in children {
        c.join().unwrap();
    }
}

fn thread_mutated_data(){
    // let mut nums = Arc::new(vec![]);
    // for n in 0..5{
    //     let mut ns = nums.clone();
    //     thread::spawn(move ||{
    //         nums.push(n);
    //     });
    // }
}

fn thread_mutex(){
    let m = Mutex::new(0);
    let c = thread::spawn(move || {
        *m.lock().unwrap() += 1;
        *m.lock().unwrap()
    });

    let updated = c.join().unwrap();
    println!("Mutex: {}", updated);
}

//Mutex locks regardless read/write
fn thread_mutex_multithreaded(){
    let data = Arc::new(Mutex::new(vec![]));
    let mut children = vec![];
    for i in 0..5 {
        let  data = data.clone();
        let t = thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data.push(i);
        });
        children.push(t);
    }

    for handle in children {
        handle.join().unwrap();
    }
    println!("{:?}", data);
}

//RwLock locks when in write mode
fn thread_rwlock(){
    let m = RwLock::new(5);
    let c = thread::spawn(move || {
        *m.write().unwrap() +=1;
        *m.read().unwrap()
    });

    let updated = c.join().unwrap();
    println!("{:?}", updated);
}

fn main() {
    thread_basics();
    thread_customize();
    thread_data();
    thread_mutated_data();
    thread_mutex();
    thread_mutex_multithreaded();
    thread_rwlock()
}
