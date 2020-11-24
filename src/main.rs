use std::env;
use std::io::{stdin, stdout, Write};
use std::process::exit;

use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::*;
use whoami;

use g_shell::built_in_command;

fn main() {
    sh_loop();
}

fn sh_loop() {
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
        
    }
}

fn argvs_execute(argvs: &Vec<&str>) -> Result<(), String> {
    match built_in_command::cd::run_cd(argvs) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }

    match built_in_command::exit::run_exit(argvs) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }

    match sh_launch() {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }
    return Ok(());
}

fn sh_launch() -> Result<(), String> {
    //子プロセスの生成
    match unsafe { fork() } {
        //親プロセス
        Ok(ForkResult::Parent { child, .. }) => {
            let pid: i32 = child.into();

            //プロセスが起動できていなければエラー
            if pid == 0 {
                return Err(format!("Error process"));
            } else if pid < 0 {
                return Err(format!("Error forking"));
            } else {
                //子プロセスを待つ
                match waitpid(child, Some(WaitPidFlag::WUNTRACED)) {
                    Ok(status) => match status {
                        WaitStatus::Exited(_, _) => {
                            println!("Exited");
                        }

                        WaitStatus::Stopped(_, _) => {
                            println!("Stopped");
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
        //子プロセス
        Ok(ForkResult::Child) => {
            exit(1);
        }

        Err(_) => {
            return Err(format!("Fork Failed"));
        }
    }

    return Ok(());
}
