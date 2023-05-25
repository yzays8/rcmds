use std::env;
use std::io;
use std::io::{Read, Write};
use std::process;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
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

    for arg in args[1..].into_iter() {
        match fs::read_to_string(arg) {
            Ok(content) => print!("{content}"),
            Err(err) => {
                eprintln!("Error reading file: {err}");
                process::exit(1);
            }
        };
    }
}
