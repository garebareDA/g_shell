use std::ffi::{CStr, CString};
use std::process::exit;

use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::*;

use super::super::built_in_command;
use super::super::parser;
use super::process::Process;

impl Process {
    pub fn argvs_execute(&mut self) -> Result<(), String> {
        let command = self.get_run_command();
        let commands = self.get_run_command().get_command();
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
            self.signal_action();
            let command = self.get_run_command().clone();
            match self.sh_launch(&command) {
                Ok(_) => {
                    for pid in self.get_process() {
                        match self.wait_process(*pid) {
                            Ok(_) => {}
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        return Ok(());
    }

    fn sh_launch(&mut self, command: &parser::parser::CommandParse) -> Result<(), String> {
        let is_empty = !self.is_empty_pipes();
        match command.get_pipe() {
            Some(_) => match pipe() {
                Ok(pipe) => {
                    self.push_pipe(pipe);
                }
                Err(_) => {
                    return Err(format!("Pipe error"));
                }
            },
            None => {}
        }
        //プロセスの生成
        match unsafe { fork() } {
            //親プロセス
            Ok(ForkResult::Parent { child, .. }) => {
                self.push_process(child);
                match command.get_pipe() {
                    Some(pipe) => match self.sh_launch(&pipe) {
                        Ok(()) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    },
                    None => {}
                }

                if is_empty {
                    match self.pearent_connect_end() {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
            //子プロセス
            Ok(ForkResult::Child) => unsafe {
                if command.get_pipe().is_some() && self.len_pipes() == 0 {
                    match self.pipe_first_connect() {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{}", e);
                            exit(-1);
                        }
                    }
                } else if !self.is_empty_pipes() && !command.get_pipe().is_some() {
                    match self.pipe_end_connect() {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{}", e);
                            exit(-1);
                        }
                    }
                } else if !self.is_empty_pipes() && command.get_pipe().is_some() {
                    match self.pipe_route_connect() {
                        Ok(_) => {}
                        Err(e) => {
                            println!("{}", e);
                            exit(-1);
                        }
                    }
                }

                let cstring = CString::new(command.get_command()).expect("CString::new failed");
                let cstr = CStr::from_bytes_with_nul_unchecked(cstring.to_bytes_with_nul());
                let mut argv: Vec<CString> = Vec::new();
                self.push_argv(&mut argv);
                let result = execvp(cstr, &argv);
                match result {
                    Ok(_) => {
                        exit(0);
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

    fn push_argv(&self, argvs: &mut Vec<CString>) {
        let command = self.get_run_command();
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

    fn wait_process(&self, child: Pid) -> Result<(), String> {
        match waitpid(child, Some(WaitPidFlag::WCONTINUED)) {
            Ok(status) => match status {
                WaitStatus::Exited(_, _) => {}

                WaitStatus::Stopped(_, _) => {}
                _ => {
                    return Err(format!("Waiprocess EOF"));
                }
            },
            Err(_) => {}
        }
        return Ok(());
    }
}
