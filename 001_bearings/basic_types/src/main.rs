/*
Three different types of pointers:
    1. References (&String is reference to a String value - "ref String")
       - references can point to stack or heap
       - &x produces ref to x or borrows a reference to x - as such *x is value of x
       - rust refs are never null and are immutable by default
       - to make mutable you's say &mut x
    2. Boxes
       - simples way to allocate a value in heap
            let t = (12, "egg")
            let b = Box::new(t);
    3. Unsafe pointers
       - rust has raw pointers like C *mut T
       - raw pointers can be null
       - can only be dereferenced in unsafe block
    4. Reference counted pointers Rc and Arc - multiple
       - Arc is thred safe A stands for Atmoic reference count
       - These are like Python refernce counted
       - Value owned by Rc is immutable - Rust law - shared/borrowed cant be mutated
       - Can be made mutable using Interior Mutability - but this could be bad
});

Arrays [T; N] - type T with N elements - constant size at compile time
Vector Vec<V> - allocated in heap is rezieable in run time
Slice &[T] or &mut [T] - slice of Ts - slice is pointer to first element + count
v.len(), v[0] applies to any of the above three types
index can only be of type usize


*/
use std::rc::Rc;

fn arrays() {
    let lazy_caterer: [u32; 6] = [1, 2, 3, 4, 5, 6];
    assert_eq!(lazy_caterer[3], 4);
    assert_eq!(lazy_caterer.iter().fold(0, |a, b| a + b), 21);

    let taxonomy = ["atul", "priya", "neil", "diya"];
    assert_eq!(taxonomy.len(), 4);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);
}
fn vectors() {
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);
    assert_eq!((0..6).fold(1, |a, b| a + b), 16);

    #[derive(Debug)]
    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}
fn replace() {
    struct Person {
        name: Option<String>,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });
    //let first_name = std::mem::replace(&mut composers[0].name, None);
    let first_name = composers[0].name.take();
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);
}
fn refcounts() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
}
fn main() {
    arrays();
    vectors();
    replace();
    refcounts();
}
