#[derive(Debug, Clone)]
pub struct Redirect {
  path: String,
  is_over:bool
}

impl Redirect {
  pub fn new(path:&str, is_over:bool) -> Self {
    Self {
      path:path.to_string(),
      is_over,
    }
  }

  pub fn get_redirect_path(&self) -> &str {
    &self.path
  }

  pub fn get_is_over(&self) -> bool {
    self.is_over
  }
}