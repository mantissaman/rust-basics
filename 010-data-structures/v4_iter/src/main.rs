pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    let mut n =0;

    loop {
        n += 1;
        if n >3 {
            break;
        }
        println!("Loop, {}!", n);
    }
    println!("Loop done.");

    n = 0;
    while n < 3 {
        n += 1;
        println!("While, {}!", n);
    }
    println!("While done.");

    for i in 0..3 {
        println!("For, {}!", i+1);
    }

    println!("For done.");

    let mut st  = Stepper{curr: 2, step: 3, max: 15};

    loop {
        match st.next() {
            Some(v) => println!("Loop Stepper, {}!", v),
            None => break
        }
    }
    println!("Loop Stepper done.");

    let mut st  = Stepper{curr: 3, step: 4, max: 15};
    while let Some(v) = st.next(){
        println!("While Stepper, {}!", v);
    }
    println!("While Stepper done.");

    let  st  = Stepper {curr: 5, step: 10, max: 55};
    for i in  st {
        println!("For Stepper, {}!", i);
    }
    println!("For Stepper done.");
}
