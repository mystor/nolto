use std::env::args;
use std::process::{Command, exit};

fn main() {
    let mut a = vec![];
    let mut it = args();
    // Remove the first argument, as it is just a reference to the current
    // executable.
    it.next().unwrap();
    // The executable we're going to want to run is the next argument.
    let rustc = it.next().expect("Expected rustc executable name.");
    while let Some(arg) = it.next() {
        if arg == "-C" {
            if let Some(arg2) = it.next() {
                if arg2 == "lto" {
                    continue;
                }

                a.push(arg);
                a.push(arg2);
                continue;
            }
        }
        a.push(arg);
    }

    let status = Command::new(rustc)
        .args(&a)
        .status()
        .expect("Failed to run rustc subprocess");

    match status.code() {
        Some(code) => exit(code),
        None => panic!("Subprocess terminated by signal"),
    }
}
