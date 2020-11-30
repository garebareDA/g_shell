use std::ffi::{CStr, CString};
use std::process::exit;

use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::*;

use super::super::built_in_command;
use super::super::parser;
use super::signal;

pub fn argvs_execute(command: &parser::parser::CommandParse) -> Result<(), String> {
    let commands = command.get_command();
    if commands == "cd" {
        match built_in_command::cd::run_cd(command) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }
    } else if commands == "exit" {
        match built_in_command::exit::run_exit(command) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        match sh_launch(command) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
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
                signal::signal_action();
                //子プロセスを待つ
                match waitpid(child, Some(WaitPidFlag::WUNTRACED)) {
                    Ok(status) => match status {
                        WaitStatus::Exited(_, _) => {}

                        WaitStatus::Stopped(_, _) => {}
                        _ => {
                            return Err(format!("Waiprocess EOF"));
                        }
                    },
                    Err(_) => {}
                }
            }
        }
        //子プロセス
        Ok(ForkResult::Child) => unsafe {
            let cstring = CString::new(format!("/bin/{}", command.get_command()))
                .expect("CString::new failed");
            let cstr = CStr::from_bytes_with_nul_unchecked(cstring.to_bytes_with_nul());
            let mut argv: Vec<CString> = Vec::new();
            push_argv(&mut argv, command);
            let result = execv(cstr, &argv);
            match result {
                Ok(_) => {
                    exit(1);
                }

                Err(_) => {
                    println!("{}: command not found", command.get_command());
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

fn push_argv(argvs: &mut Vec<CString>, command: &parser::parser::CommandParse) {
    argvs.push(CString::new(command.get_command()).expect("CString::new failed"));
    if command.get_sub_command() != "" {
        argvs.push(CString::new(command.get_sub_command()).expect("CString::new failed"));
    } else if command.get_path() != "" {
        argvs.push(CString::new(command.get_path()).expect("CString::new failed"));
    }

    for option in command.get_options() {
        argvs.push(CString::new(option.to_string()).expect("CString::new failed"));
    }
}
