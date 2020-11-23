use std::process::exit;

pub fn run_exit(argvs:&Vec<&str>) -> Result<(), String> {
  if argvs[0] == "exit" {
    if argvs.len() == 1 {
      exit(1);
    }else{
      return Err(format!("exit has no subcommands and options"));
    }
  }
  return Ok(());
}