//Thread unsafe reference counting smart pointers
//Use Cell or RefCell for interior mutability
//Must be on heap
use crate::cell::Cell;
use std::ptr::NonNull;
use std::marker::PhantomData;

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

pub struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });
        Rc {
            inner: unsafe{ NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc { inner: self.inner, 
            _marker: PhantomData, 
        }
    }
}
impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}
impl<T> Drop for Rc<T>{
    fn drop(&mut self){
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c==1 {
            //last RC - drop
            drop(inner);
            let _= unsafe { Box::from_raw(self.inner.as_ptr())} ;
        } else {
            //Dont drop the box
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bad() {
        let (x, _y);
        x = String::from("foo");
        _y = Rc::new(&x);
    }
}