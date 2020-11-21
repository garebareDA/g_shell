use std::io::{Write, stdout, stdin};
use std::env;

use whoami;
use dirs;

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

        match argvs_execute(&line_split) {
            Ok(()) => {}
            Err(e) => {
                eprint!("{}", e);
            }
        }
    }
}


fn argvs_execute(argvs:&Vec<&str>) -> Result<(), String> {
    if argvs[0] == "cd" {
        if argvs.len() != 2 {
            env::set_current_dir(dirs::home_dir().unwrap()).unwrap();
            return Ok(());
        }else if !env::set_current_dir(argvs[1]).is_ok() {
           return Err(format!("cd {}: No such file or directory\n", argvs[1]));
        }
    }
    return Ok(());
}