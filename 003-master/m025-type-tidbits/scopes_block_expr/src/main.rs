
fn block_expr() {
    let precompute = {
        let a = (-34i64).abs();
        let b = 345i64.pow(3);
        let c=3;
        a +b +c
    };

    let result_msg = match precompute {
        42 => "done",
        a if a%2 ==0 => "continue", //This is guard
        _ => panic!("oh crap!")
    };

    println!("{}", result_msg);

}

fn compute(i:i32) -> i32{
    2 * i
}
fn if_expr(){
    let result_msg  ="done";

    let result = if result_msg=="done" {
        let _some_work = compute(8);
        let stuff = compute(4);
        compute(2) + stuff
    }
    else{
        compute(1)
    };

    println!("{}", result);
}

fn main() {
    block_expr();
    if_expr();

    //declare not initialize
    let  a: i32;
    // This is going to fail
    // println!("{}", a);
    a = 23;
    println!("{}", a);
}
