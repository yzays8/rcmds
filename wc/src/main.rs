use std::env;
use std::fs;
use std::io;
use std::io::{Read, BufReader};
use std::process;

struct Counts {
    nlines: u32,
    nwords: u32,
    nbytes: u32,
}

fn main() {
    let args = env::args();
    let argc = args.len();
    let mut counts = Counts {
        nlines: 0,
        nwords: 0,
        nbytes: 0,
    };

    if argc == 1 {
        wc(&mut io::stdin(), "", &mut counts);
        return;
    }

    for arg in args.skip(1) {
        let mut fd = fs::File::open(&arg).unwrap_or_else(|err| {
            eprintln!("Error reading file: {err}");
            process::exit(1);
        });

        wc(&mut fd, &arg, &mut counts);
    }

    if argc > 2 {
        println!("{:>2} {:>2} {:>2} total", counts.nlines, counts.nwords, counts.nbytes);
    }
}

fn wc(fd: &mut dyn Read, filename: &str, counts: &mut Counts) {
    let mut buf: [u8; 4096] = [0; 4096];
    let mut inword = false;
    let mut nl = 0;
    let mut nw = 0;
    let mut nb = 0;
    let mut reader = BufReader::new(fd);

    loop {
        match reader.read(&mut buf) {
            Ok(0) => {
                if filename.is_empty() {
                    println!("{:>2} {:>2} {:>2}", nl, nw, nb);
                } else {
                    println!("{:>2} {:>2} {:>2} {:>2}", nl, nw, nb, filename);
                }
                counts.nlines += nl;
                counts.nwords += nw;
                counts.nbytes += nb;
                break;
            },
            Ok(n) => {
                nb += n as u32;
                for c in &buf[..n] {
                    let c = *c as char;
                    if c == '\n' {
                        nl += 1;
                    }

                    if inword && !c.is_alphabetic() {
                        inword = false;
                    } else if !inword && c.is_alphabetic() {
                        inword = true;
                        nw += 1;
                    }
                }
            },
            Err(err) => {
                eprintln!("Error reading file: {err}");
                process::exit(1);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wc_literal() {
        let mut counts = Counts {
            nlines: 0,
            nwords: 0,
            nbytes: 0,
        };
        let mut fd = io::Cursor::new("hello world\n");

        wc(&mut fd, "", &mut counts);
        assert_eq!(counts.nlines, 1);
        assert_eq!(counts.nwords, 2);
        assert_eq!(counts.nbytes, 12);
    }

    #[test]
    fn test_wc_file() {
        let mut counts = Counts {
            nlines: 0,
            nwords: 0,
            nbytes: 0,
        };
        let mut fd = fs::File::open("test.txt").unwrap();
        wc(&mut fd, "test.txt", &mut counts);
        assert_eq!(counts.nlines, 5);
        assert_eq!(counts.nwords, 8);
        assert_eq!(counts.nbytes, 32);
    }
}
