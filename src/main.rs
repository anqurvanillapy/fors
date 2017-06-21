use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;

fn log_fatal(e: Error) {
    println!("{}", e);
    process::exit(1);
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let fpath = Path::new(&arg1);
        match  File::open(&fpath) {
            Ok(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut src = String::new();

                match buf_reader.read_to_string(&mut src) {
                    Ok(_) => println!("src={}", src),
                    Err(e) => log_fatal(e)
                }
            },
            Err(e) => log_fatal(e)
        }
    } else {
        println!("Usage: fors filename");
        process::exit(1);
    }
}
