#[derive(Debug, Clone)]
pub struct CommandParse {
  pub command: String,
  pub sub_command: String,
  pub option: Vec<String>,
  pub path:String,
  pub index:usize,
}

impl CommandParse {
  pub fn new() -> Self {
    Self {
      command: String::new(),
      sub_command: String::new(),
      option: Vec::new(),
      path:String::new(),
      index:0,
    }
  }

  pub fn run(&mut self, line: String) {
    let line_split: Vec<&str> = line.split(" ").collect();
    let line_index = line_split.len();
    self.command = line_split[0].to_string();
    self.index += 1;

    loop {
      if line_index <= self.index {
        break;
      }
      self.judge(line_split[self.index]);
      self.index += 1;
    }
  }

  fn judge(&mut self, args:&str) {
    if args.contains("-") {
      self.option.push(args.to_string());
      return;
    }

    if args.contains("/") || args.contains(".") {
      self.path = args.to_string();
      return;
    }

    self.sub_command = args.to_string();
  }
}
