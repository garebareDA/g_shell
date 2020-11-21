use whoami;

use std::io::{Write, stdout, stdin};
use std::env;

fn main() {

    loop {
        print!("{}@{}:{} > ",whoami::username(), whoami::hostname(), env::current_dir().unwrap().display());
        stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Faild to read line");
    }
}
