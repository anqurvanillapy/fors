use std::env;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let fpath = Path::new(&arg1);
        if let Ok(file) = File::open(&fpath) {
            let mut buf_reader = BufReader::new(file);
            let mut src = String::new();

            if let Ok(_) = buf_reader.read_to_string(&mut src) {
                println!("src={}", src);
            }
        }
    } else {
        println!("Usage: fors filename");
        process::exit(1);
    }
}
