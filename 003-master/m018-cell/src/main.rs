use std::cell::Cell;
use std::cell::RefCell;

#[derive(Debug)]
struct Bag {
    item: Box<u32>
}

fn main() {
    let bag = Cell::new(Bag { item: Box::new(1)});
    let hand1 = &bag;
    let hand2 = &bag;

    hand1.set(Bag { item: Box::new(2)});
    hand2.set(Bag { item: Box::new(3)});

    //Only unwrap Copy
    //let borrowed = hand1.get();

    let bag = RefCell::new(Bag { item: Box::new(1)});
    let hand1 = &bag;
    let hand2 = &bag;

    *hand1.borrow_mut() = Bag { item: Box::new(2)};
    *hand2.borrow_mut() = Bag { item: Box::new(3)};

    let borrowed = hand1.borrow();
    println!("{:?}", borrowed);
}
