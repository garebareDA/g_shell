pub struct CommandParse {
  pub command: String,
  pub sub_command: String,
  pub option: Vec<String>,
  pub index:usize,
}

impl CommandParse {
  pub fn new() -> Self {
    Self {
      command: String::new(),
      sub_command: String::new(),
      option: Vec::new(),
      index:0,
    }
  }

  pub fn run(&mut self, line: String) {
    let line_split: Vec<&str> = line.split(" ").collect();
    let line_index = line_split.len();
    loop {
      if line_index > self.index {
        break;
      }
      self.index += 1;
    }
  }
}
