/*
to compile: cargo run
*/
#![allow(warnings)]
use std::time::Instant;

use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

mod CGwat;
mod P0;
mod SC;
mod ST;
mod P0test;

fn main() {
    let now = Instant::now();
    P0test::testAll();
    println!("\ntotal runtime: {} milliseconds", now.elapsed().as_micros() as f64/1000.000);
}
