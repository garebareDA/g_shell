use std::env;
use std::io::{stdin, stdout, Write};
use std::process::exit;

use nix::sys;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::*;

use dirs;
use whoami;

fn main() {
    loop {
        print!(
            "{}@{}:{} > ",
            whoami::username(),
            whoami::hostname(),
            env::current_dir().unwrap().display()
        );
        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).expect("Faild to read line");
        line.remove(line.len() - 1);
        let line_split: Vec<&str> = line.split(" ").collect();

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

fn argvs_execute(argvs: &Vec<&str>) -> Result<(), String> {
    if argvs[0] == "cd" {
        if argvs.len() != 2 {
            env::set_current_dir(dirs::home_dir().unwrap()).unwrap();
            return Ok(());
        } else if !env::set_current_dir(argvs[1]).is_ok() {
            return Err(format!("cd {}: No such file or directory\n", argvs[1]));
        }
    }
    return Ok(());
}

fn sh_launch() -> Result<(), String> {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            let pid: i32 = child.into();
            if pid == 0 {
                return Err(format!("Error process"));
            } else if pid < 0 {
                return Err(format!("Error forking"));
            } else {
                loop {
                    match waitpid(child, Some(WaitPidFlag::WUNTRACED)) {
                        Ok(status) => match status {
                            WaitStatus::Exited(_, _) => {
                                break;
                            }

                            WaitStatus::Stopped(_, _) => {
                                break;
                            }
                            _ => {
                                return Err(format!("Waiprocess EOF"));
                            }
                        },
                        Err(_) => {
                            return Err(format!("Waitprocess error"));
                        }
                    }
                }
            }
        }
        Ok(ForkResult::Child) => println!("I'm a new child process"),
        Err(_) => {
            return Err(format!("Fork Failed"));
        }
    }

    return Ok(());
}
