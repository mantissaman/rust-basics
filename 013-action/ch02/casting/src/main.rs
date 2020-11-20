
use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: i64 = 100;

    if (a as i64 ) < b {
        println!("a is less than b");
    }
    //or

    if a < b.try_into().unwrap()  {
        println!("a is less than b");
    }

    assert!(0.1 + 0.3 == 0.4);

    //under the hood
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let difference = desired - result;
    println!("desired - result = {}", difference);
    let absolute_difference = difference.abs();
    assert!(absolute_difference <= f32::EPSILON);
}
