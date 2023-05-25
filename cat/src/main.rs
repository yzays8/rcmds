use std::env;
use std::io;
use std::io::{Read, Write};
use std::process;
use std::fs;

fn main() {
    let mut args = env::args();
    if args.len() == 1 {
        let mut buf = [0; 4096];
        loop {
            match io::stdin().read(&mut buf) {
                Ok(n) => io::stdout().write(&buf[0..n]).unwrap_or_else(|err| {
                    eprintln!("Error writing stdout: {err}");
                    process::exit(1);
                }),
                Err(err) => {
                    eprintln!("Error reading stdin: {err}");
                    process::exit(1);
                },
            };
        }
    }

    args.next();    // skip program name
    for arg in args {
        match fs::read_to_string(arg) {
            Ok(content) => print!("{content}"),
            Err(err) => {
                eprintln!("Error reading file: {err}");
                process::exit(1);
            }
        };
    }
}
