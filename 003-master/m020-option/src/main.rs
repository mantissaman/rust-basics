use std::collections::HashMap;

fn main() {
    let mut map = HashMap::<&str, u32>::new();
    map.insert("one", 1);
    map.insert("two", 2);

    let value = map.get("one");
    // let incremented_value = value + 1;
    let incremented_value = match value {
        Some(val) => val +1,
        None => 0
    };

    println!("{}", incremented_value);

    let incremented_value = if let Some(v) = map.get("two") {
        v +1
    } 
    else{
        0
    };
    println!("{}", incremented_value);

    let incremented_value = map.get("two").unwrap();
    println!("{}", incremented_value);

    let incremented_value = map.get("three").expect("Key not found");
    println!("{}", incremented_value);
}
