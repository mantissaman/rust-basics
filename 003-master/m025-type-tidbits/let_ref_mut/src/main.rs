
#[derive(Debug)]
struct Items(i32);

fn main() {
    let items = Items(2);
    let items_ptr = &items;
    let ref items_ref = items; //its same as the line above

    //Cast to *const Items is a raw pointer type to items.
    assert_eq!(items_ptr as *const Items, items_ref as *const Items);

    let mut a = Items(20);

    let ref mut b = a; // same as let mut b = &a;
    b.0 += 25;

    print!("{:?}", items);
    print!("{:?}", a);
}
