use serde::{Serialize, Deserialize};
use std::{fs, path};

use crate::{errors::{Result, ScuError}, interpreter::Interpreter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shortcut {
  pub name: String,
  pub interpreters: Option<Vec<Interpreter>>,
  pub body: ShortcutBody
}

impl Shortcut {
  pub fn load(path: impl AsRef<path::Path>) -> Result<Self> {
    toml::from_str(fs::read_to_string(path)?.as_str())
      .map_err(|err| err.into())
  }

  pub fn store(&self, path: impl AsRef<path::Path>) -> Result<()> {
    toml::to_string_pretty(self).map_err(|err| ScuError::from(err))
      .and_then(|data| fs::write(path, data).map_err(|err| err.into()))
  }

  pub fn builder() -> ShortcutFileBuilder {
    ShortcutFileBuilder::new()
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ShortcutBody {
  Command(Vec<String>),
}

impl ShortcutBody {
  pub fn command(&self) -> Vec<String> {
    match self {
      Self::Command(vec) => vec.clone()
    }
  }
}

impl std::fmt::Display for ShortcutBody {
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
  pub interpreters: Option<Vec<Interpreter>>,
  pub body: Option<ShortcutBody>
}

impl ShortcutFileBuilder {
  pub fn new() -> Self {
    ShortcutFileBuilder {
      name: None,
      interpreters: None,
      body: None
    }
  }

  pub fn name(mut self, name: impl AsRef<str>) -> Self {
    self.name = Some(name.as_ref().into());
    self
  }
  
  pub fn interpreters(mut self, interpreters: Option<Vec<Interpreter>>) -> Self {
    self.interpreters = interpreters;
    self
  }

  pub fn command(mut self, command: Vec<String>) -> Self {
    self.body = Some(ShortcutBody::Command(command));
    self
  }

  pub fn build(self) -> Shortcut {
    Shortcut {
      name: self.name.unwrap(),
      interpreters: self.interpreters,
      body: self.body.unwrap()
    }
  }
}