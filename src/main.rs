use whoami;

use std::io::{Write, stdout, stdin};
use std::env;

fn main() {

    loop {
        print!("{}@{}:{} > ",whoami::username(), whoami::hostname(), env::current_dir().unwrap().display());
        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).expect("Faild to read line");
        line.remove(line.len() - 1);
        let line_split:Vec<&str> = line.split(" ").collect();

        if line_split[0] == "exit" {
            return;
        }
    }
}
