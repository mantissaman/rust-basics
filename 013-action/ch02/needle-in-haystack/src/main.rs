use std::time::{Duration, Instant};

fn main() {
    let needle = 0o52;
    let haystack = [1,1,2,5,14,42,132,429,1430,4862];

    //&haystak points to array it enables to iterate
    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }

    //or - this is better - iterate through pointer to element
    for item in haystack.iter() {
        if *item == needle {
            println!("{}", item);
        }
    }
    let haystack = vec![1,1,2,5,14,42,132,429,1430,4862];

    //into_iter - iterate through values of elements
    for item in haystack.into_iter() {
        if item == needle {
            println!("{}", item);
        }
    }

    for _ in 0..4 {
        println!("###");
    }

    for _ in 0..=4 {
        println!("***");
    }

    // not ver idiomatic - incurs runtime costs and may be unsafe
    let collection = [1,2,3,4,5];
    for i in 0..collection.len() {
        println!("{}", collection[i]);
    }

    let mut count =0;
    let time_limit = Duration::new(1,0);
    let start = Instant::now();

    while(Instant::now() - start) < time_limit {
        count+=1;
    }
    println!("loops {}", count);

    //shouldn't use while for endless loop use loop instead
    'outer: loop {
        for y in 0 .. {
            for z in 0 .. {
                if y + z > 1000 {
                    break 'outer;
                }
            }
        }
    }
}
