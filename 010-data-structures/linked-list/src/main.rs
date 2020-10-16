mod lib;

fn main() {
    let mut tl =lib::TransactionLog::new_empty();
    tl.append("Hello".to_owned());
    println!("{:?}", tl);
    let x =tl.pop();
    println!("{}",x.unwrap());
    println!("{:?}", tl);
}
