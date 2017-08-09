use std::process::{Command, exit};

pub fn run_sub<I: Iterator<Item = String>>(rustc: &str, mut it: I) {
    let mut a = vec![];
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
