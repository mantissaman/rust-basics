//Safe dynamically checked borrow checking.
//Good for graphs and trees
use std::cell::UnsafeCell;

enum RefSate{
    Unshared,
    Shared(usize),
    Exclusive
}

pub struct RefCell<T>{
    value : UnsafeCell<T>,
    state: RefSate, //how many shared reference?
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell{
            value: UnsafeCell::new(value),
            state: RefSate::Unshared
        }
    }

    pub fn borrow(&self) -> Option<&T> {
        match self.state {
            RefSate::Unshared => Some(unsafe { &mut *self.value.get()}),
            RefSate::Shared(_) => Some(unsafe { &mut *self.value.get()}),
            RefSate::Exclusive => None
        }
    }
    pub fn borrow_mut(&self) -> Option<&mut T> {
        if let RefSate::Unshared = self.state {
            Some(unsafe { &mut *self.value.get()})
        }
        else{
            None
        } 
    }
}