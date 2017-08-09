extern crate nolto;

use std::env;
use nolto::run_sub;

fn main() {
    let mut it = env::args();
    it.next().unwrap(); // Discard executable name
    let rustc = it.next().expect("Expected rustc executable name.");
    run_sub(&rustc, it);
}
