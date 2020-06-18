//Raw type
// Only way to go for shared reference
use std::cell::UnsafeCell;

//Used for small Copy values
//flags, counts etc. - thread local
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// Implid by Unsafe
// impl<T> !Sync for Cell<T>{}

//total unsafe - all hell is loose
//unsafe impl<T> Sync for Cell<T>{}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // No one else is concurrently mutation this value
        // not invalidating any references becuse we are not giving any out
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // No one else is modying this value
        // only this thread can mutate (!Sync)
        unsafe { *self.value.get() }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::Cell;
//     use std::sync::Arc;
//     use std::thread;

//     #[test]
//     fn bad() {
//         let x = Arc::new(Cell::new(0));
//         let x1 = Arc::clone(&x);
//         let h = thread::spawn(move || {
//             for _ in 0..100000 {
//                 let x = x1.get();
//                 x1.set(x + 1);
//             }
//         });
//         let x2 = Arc::clone(&x);
//         let h1 = thread::spawn(move || {
//             for _ in 0..100000 {
//                 let x = x2.get();
//                 x2.set(x + 1);
//             }
//         });
//         h.join().unwrap();
//         h1.join().unwrap();

//         assert_eq!(x.get(), 200000);
//     }
//     #[test]
//     fn bad2() {
//         let x = Cell::new(42);
//         let first = x.get();
//         x.set(30);
//         eprintln!("{}", first);
//     }
// }
