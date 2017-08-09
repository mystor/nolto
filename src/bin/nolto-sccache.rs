extern crate nolto;

use std::env;
use nolto::run_sub;

fn main() {
    let mut it = env::args();
    it.next().unwrap(); // Discard executable name
    let sccache = env::var("SCCACHE").unwrap_or("sccache".to_owned());
    run_sub(&sccache, it);
}
