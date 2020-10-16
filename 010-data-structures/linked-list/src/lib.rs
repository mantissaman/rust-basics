use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
pub struct Node {
    pub value: String,
    pub next: SingleLink
    //RC RefCell - so that we can retrieve and replace - Interior Mutability
}

impl Node {
    pub fn new(value: String) -> Rc<RefCell<Node>>{
        Rc::new(RefCell::new(Node{
                    value:value,
                    next: None,
                }))
    }
}
#[derive(Debug)]
pub struct TransactionLog {
    pub head: SingleLink,
    pub tail: SingleLink,
    pub length: u64
}

impl TransactionLog {
    pub fn new_empty() ->TransactionLog{
        TransactionLog{
            head: None,
            tail: None,
            length: 0
        }
    }
    pub fn append(&mut self, value:String){
        //create the new node
        let new = Node::new(value);

        match self.tail.take() {
            //If already a tail then add new as next of tail
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            //If No tail then add new as head
            None => self.head = Some(new.clone())
        };
        self.length +=1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String>{
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take(){
                self.head = Some(next);
            } else{
                self.tail.take();
            }
            self.length -=1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}