use serde::{Serialize, Deserialize};
use std::{fs, io, path};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortcutFile {
  pub name: String,
  pub override_interpreters: Option<Vec<String>>,
  pub body: ShortcutFileBody
}

impl ShortcutFile {
  pub fn load(path: impl AsRef<path::Path>) -> io::Result<Self> {
    toml::from_str(fs::read_to_string(path)?.as_str())
      .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
  }

  pub fn store(&self, path: impl AsRef<path::Path>) -> io::Result<()> {
    toml::to_string_pretty(self).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
      .and_then(|data| fs::write(path, data))
  }

  pub fn builder() -> ShortcutFileBuilder {
    ShortcutFileBuilder::new()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShortcutFileBody {
  Command(Vec<String>),
}

impl std::fmt::Display for ShortcutFileBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Self::Command(cmd) => cmd.into_iter()
      .map(|str| if str.contains(" ") { format!("\"{}\"", str) } else { str.to_string() })
      .reduce(|a, b| format!("{} {}", a, b)).unwrap(),
    })
  }
}

pub struct ShortcutFileBuilder {
  pub name: Option<String>,
  pub override_interpreters: Option<Vec<String>>,
  pub body: Option<ShortcutFileBody>
}

impl ShortcutFileBuilder {
  pub fn new() -> Self {
    ShortcutFileBuilder {
      name: None,
      override_interpreters: None,
      body: None
    }
  }

  pub fn name(mut self, name: impl AsRef<str>) -> Self {
    self.name = Some(name.as_ref().into());
    self
  }
  
  pub fn override_interpreters(mut self, interpreters: Option<Vec<String>>) -> Self {
    self.override_interpreters = interpreters;
    self
  }

  pub fn command(mut self, command: Vec<String>) -> Self {
    self.body = Some(ShortcutFileBody::Command(command));
    self
  }

  pub fn build(self) -> ShortcutFile {
    ShortcutFile {
      name: self.name.unwrap(),
      override_interpreters: self.override_interpreters,
      body: self.body.unwrap()
    }
  }
}