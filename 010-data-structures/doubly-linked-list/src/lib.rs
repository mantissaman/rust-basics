use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug,Clone)]
struct Node{
    value: String,
    next: Link,
    prev: Link,
}
type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug,Clone)]
pub struct BetterTransactionLog {
    head: Link,
    tail: Link,
    pub length: u64
}