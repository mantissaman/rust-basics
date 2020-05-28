use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T>{
    data: T,
    next: Option<Rc<Node<T>>>,
    prev: RefCell<Option<Weak<Node<T>>>>
}

#[derive(Debug)]
struct LinkedList<T>{
    head: Option<Rc<Node<T>>>
}

impl<T> LinkedList<T>{
    fn new() -> Self {
        LinkedList {head: None}
    }

    fn append(&self, data: T) -> Self {
        let new_node = Rc::new(Node {
            data: data,
            next: self.head.clone(),
            prev: RefCell::new(None)
        });

        match self.head.clone() {
            Some(node) => {
                let mut prev = node.prev.borrow_mut();
                *prev = Some(Rc::downgrade(&new_node))
            },
            None => {

            }
        }

        LinkedList {
            head: Some(new_node)
        }
    }
}

fn main() {
    let list_of_nums = LinkedList::<i32>::new().append(1).append(2);
    println!("nums: {:?}", list_of_nums);

    let list_of_strs = LinkedList::new().append("foo").append("bar");
    println!("strs: {:?}", list_of_strs);
}
