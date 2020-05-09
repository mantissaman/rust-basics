use std::collections::HashMap;

static WORTH_POINTING_AT: i32 = 1000;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("Works by {}", artist);
        for work in works {
            println!("    {}", work);
        }
    }
}

fn sort_work(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}
fn deref() {
    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    };
    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");

    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");
}

fn deref_multihop() {
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 729);
}
fn implicit_borrow() {
    let mut v = vec![1973, 1968];
    v.sort(); // implicitly borrows a mutable reference to v
    (&mut v).sort(); // equivalent; much uglier
}
fn comp_deref() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    // always compaere deref values
    assert!(rrx <= rry);
    assert!(rrx == rry);
    //if you really want to check if address is same
    assert!(rx == ry); // their referents are equal
    assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses
}
fn literal_borrow() {
    fn factorial(n: usize) -> usize {
        (1..n + 1).fold(1, |a, b| a * b)
    }
    let r = &factorial(6);
    // Arithmetic operators can see through one level of references.
    assert_eq!(r + &1009, 1729);
}
fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );
    sort_work(&mut table);
    show(&table);
    deref();
    deref_multihop();
    implicit_borrow();
    comp_deref();
    literal_borrow();
    // let x =42;
    // static_lifetime(&x); /x doesnt live long enough
    static_lifetime(&WORTH_POINTING_AT);
    strct_lifetime();
    multiple_lifetimes();
    self_lifetimes();
}
/*
C/C++ references (pointers) are just addresses
Rust references have a very differnt feel
In rust refernces created with & need explict * to dereference.println.println!
Then why did we not derefernce  &table  using *table in show fn?
Rust implicity dereferences its left operand  of dot if needed. can take multiple hops.
dot operator can also implicitly borrow a ref to its left opernad
if no dot and comparision operator then no implit - use * to deref
you can even borrow references to literals - just about anything
*/
static mut STASH: &i32 = &10;

//We have to use statis as STASH lives longer than
//paramater p - override implicit fn<'a> f(p: &'a i32)
//
fn static_lifetime(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}
fn strct_lifetime() {
    struct S<'a> {
        r: &'a i32,
    }

    let s;
    //{
    let x = 10;
    s = S { r: &x };
    //}
    assert_eq!(*s.r, 10); // bad: reads from dropped `x`
}

fn multiple_lifetimes() {
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }
}

fn self_lifetimes() {
    struct StringTable {
        elements: Vec<String>,
    }

    impl StringTable {
        fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
            for i in 0..self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return Some(&self.elements[i]);
                }
            }
            None
        }
    }

    let x = StringTable { elements: vec!["Atul".to_string(),"Priya".to_string()]};
    let y=x.find_by_prefix(&"A");
    println!("{:?}", y.unwrap());
}

/*
Variable’s lifetime must contain or enclose that of the reference borrowed from it.
Reference’s lifetime must contain or enclose the variable’s.

1. If function doesn't return ref no need to specify lifetimes
2. single ref parameter to function no need for lifetime
3. multiple ref params then we have to do the circus
4. self rules
*/
