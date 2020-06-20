//Safe dynamically checked borrow checking.
//Good for graphs and trees
use std::cell::UnsafeCell;
use crate::cell::Cell;

#[derive(Debug, Copy, Clone)]
enum RefSate{
    Unshared,
    Shared(usize),
    Exclusive
}

pub struct RefCell<T>{
    value : UnsafeCell<T>,
    state: Cell<RefSate>, //how many shared reference - shared and mutable - use Cell type
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell{
            value: UnsafeCell::new(value),
            state: Cell::new(RefSate::Unshared)
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefSate::Unshared => {
                self.state.set(RefSate::Shared(1));
                Some(Ref{refcell: self})

            },
            RefSate::Shared(n) => {
                self.state.set(RefSate::Shared(n + 1));
                Some(Ref{refcell: self})

            },
            RefSate::Exclusive => None
        }
    }
    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefSate::Unshared = self.state.get() {
            self.state.set(RefSate::Exclusive);
            //SAFETY: No other refs given out since state would be shared or exclusive
            Some(RefMut{refcell: self})
        }
        else{
            None
        } 
    }
}
//Got lifetime of refcell
pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for Ref<'_, T>{
    type Target = T;
    fn deref(&self) -> &Self::Target{
        unsafe{&*self.refcell.value.get()}
    }
}

impl<T> Drop for Ref<'_, T>{
    fn drop(&mut self){
        match self.refcell.state.get(){
            RefSate::Exclusive | RefSate::Unshared => unreachable!(),
            RefSate::Shared(1) => {
                self.refcell.state.set(RefSate::Unshared);
            },
            RefSate::Shared(n) => {
                self.refcell.state.set(RefSate::Shared(n-1));
            },
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for RefMut<'_, T>{
    type Target = T;
    fn deref(&self) -> &Self::Target{
        unsafe{&*self.refcell.value.get()}
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T>{
    //type Target = T;
    fn deref_mut(&mut self) -> &mut Self::Target{
        unsafe{&mut *self.refcell.value.get()}
    }
}

impl<T> Drop for RefMut<'_, T>{
    fn drop(&mut self){
        match self.refcell.state.get(){
            RefSate::Shared(_) | RefSate::Unshared => unreachable!(),
            RefSate::Exclusive => {
                self.refcell.state.set(RefSate::Unshared);
            },
        }
    }
}