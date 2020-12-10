use super::redirect::Redirect;

#[derive(Debug, Clone)]
pub struct CommandParse {
  command: String,
  sub_command: String,
  option: Vec<String>,
  path: String,
  index: usize,
  pipe: Option<Box<CommandParse>>,
  redirect: Option<Redirect>,
}

impl CommandParse {
  pub fn new() -> Self {
    Self {
      command: String::new(),
      sub_command: String::new(),
      option: Vec::new(),
      path: String::new(),
      index: 0,
      pipe: None,
      redirect: None,
    }
  }

  pub fn run(&mut self, line: String) {
    let mut line_split: Vec<&str> = line.split(" ").collect();
    self.judge_loop(&mut line_split);
  }

  fn judge_loop(&mut self, mut line_split: &mut Vec<&str>) {
    self.index += 1;
    let line_index = line_split.len();
    self.command = line_split[0].to_string();
    loop {
      if line_index <= self.index {
        break;
      }
      self.judge(&mut line_split);
      self.index += 1;
    }
  }

  fn judge(&mut self, args: &mut Vec<&str>) {
    let arg = args[self.index];

    if arg.chars().nth(1).unwrap() == '>' {
      if arg.chars().nth(2).unwrap() == '>' {
        return;
      }


    }

    if arg.chars().nth(1).unwrap() == '-' {
      self.option.push(arg.to_string());
      return;
    }

    if arg.contains("/") || arg.contains(".") {
      self.path = arg.to_string();
      return;
    }

    if arg == "|" {
      let mut command = CommandParse::new();
      let mut args_split:Vec<&str> = Vec::new();
      for index in self.index + 1 .. args.len() {
        let split = args[index];
        args_split.push(split);
      }
      command.judge_loop(&mut args_split);
      self.pipe = Some(Box::new(command));
      self.index += args.len() - self.index;
      return;
    }

    self.sub_command = arg.to_string();
  }

  pub fn get_command(&self) -> &str {
    &self.command
  }

  pub fn get_sub_command(&self) -> &str {
    &self.sub_command
  }

  pub fn get_path(&self) -> &str {
    &self.path
  }

  pub fn get_index(&self) -> usize {
    self.index
  }

  pub fn get_options(&self) -> &Vec<String> {
    &self.option
  }

  pub fn get_pipe(&self) -> &Option<Box<CommandParse>> {
    &self.pipe
  }
}
