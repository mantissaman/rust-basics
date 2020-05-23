#![feature(test)]
extern crate test;

use test::Bencher;

pub fn do_nothing_slowly(){
    println!(".");
    for _ in 1..10_000_000 {}
}
pub fn do_nothing_fast(){
}

#[bench]
fn bench_nothing_slowly(b: &mut Bencher){
    b.iter(|| do_nothing_slowly());
}

#[bench]
fn bench_nothing_fast(b: &mut Bencher){
    b.iter(|| do_nothing_fast());
}

// rustup override set nightly