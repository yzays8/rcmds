use std::env;

fn unescaped(arg: String) {
    let s = arg.chars();
    let mut back_slashed = false;

    for c in s {
        if c != '\\' && !back_slashed {
            print!("{}", c);
            continue;
        } else {
            if back_slashed {
                match c {
                    'n' => print!("\n"),
                    'r' => print!("\r"),
                    't' => print!("\t"),
                    '0' => break,
                    _ => print!("\\{c}"),
                }
                back_slashed = false;
            } else {
                back_slashed = true;
            }
        }
    }
}

fn main() {
    let mut args = env::args();
    let argc = args.len();

    if argc == 1 {
        println!("");
        return;
    }

    args.next();

    let mut first = true;
    match args.next().unwrap().as_str() {
        "-e" => {
            for arg in args {
                if !first {
                    print!(" ");
                }
                unescaped(arg);
                first = false;
            }
            println!("");
        }
        "-n" => {
            for arg in args {
                if !first {
                    print!(" ");
                }
                print!("{arg}");
                first = false;
            }
        }
        arg1 => {
            print!("{arg1}");
            for arg in args {
                print!(" {arg}");
            }
            println!("");
        }
    }
}
