use std::env;
use std::ffi::{CStr, CString};
use std::io::{stdin, stdout, Write};
use std::process::exit;

use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::*;
use whoami;

use g_shell::built_in_command;
use g_shell::parser;

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
        let mut command = parser::parser::CommandParse::new();
        command.run(line);
        match argvs_execute(&command) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

fn argvs_execute(command: &parser::parser::CommandParse) -> Result<(), String> {
    match built_in_command::cd::run_cd(command) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }

    match built_in_command::exit::run_exit(command) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }

    match sh_launch(command) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }
    return Ok(());
}

fn sh_launch(command: &parser::parser::CommandParse) -> Result<(), String> {
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
        Ok(ForkResult::Child) => unsafe {
            let cstring = CString::new(format!("/bin/{}", command.command)).expect("CString::new failed");
            let cstr = CStr::from_bytes_with_nul_unchecked(cstring.to_bytes_with_nul());
            let mut args: Vec<CString> = Vec::new();
            args.push(CString::new("").expect("CString::new failed"));
            let result = execv(cstr, &args);
            match result {
                Ok(infa) => {
                    println!("{:?}", infa);
                    exit(1);
                }

                Err(e) => {
                    exit(-1);
                }
            }
        },

        Err(_) => {
            return Err(format!("Fork Failed"));
        }
    }

    return Ok(());
}
