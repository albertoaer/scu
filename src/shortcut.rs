use serde::{Serialize, Deserialize};
use std::{fs, path, ops::Deref};

use crate::{errors::{Result, ScuError}, interpreter::Interpreter, paths, startup::StartupReference, script::Script};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shortcut {
  pub name: String,
  pub interpreters: Option<Vec<Interpreter>>,
  pub body: ShortcutBody,
  pub startup: Option<StartupReference>
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

  pub fn builder() -> ShortcutBuilder {
    ShortcutBuilder::new()
  }
  
  pub fn script<'a>(&self, interpreter: &'a Interpreter) -> Result<Script<'a>> {
    Script::new(interpreter, self.command())
  }

  pub fn update_startup_reference(&mut self, startup: Option<StartupReference>) {
    self.startup = startup
  }
}

impl Deref for Shortcut {
  type Target = ShortcutBody;

  fn deref(&self) -> &Self::Target {
    &self.body
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "command")]
pub enum ShortcutBody {
  Command(Vec<String>),
  CommandWithScript {
    cmd: Vec<String>,
    script: path::PathBuf,
    script_offset: Option<u8>,
    body: String,
  }
}

impl ShortcutBody {
  pub fn command(&self) -> Vec<String> {
    match self {
      Self::Command(cmd) => cmd.clone(),
      Self::CommandWithScript { cmd, script, body: _, script_offset } => {
        let mut command = cmd.clone();
        command.insert(script_offset.unwrap_or(command.len() as u8).into(), paths::stringify(script, "\\\\"));
        command
      },
    }
  }

  pub fn write_resources(&self) -> Result<()> {
    match self {
      Self::CommandWithScript { cmd: _, script, body, script_offset: _ } => fs::write(script, body).map_err(|err| err.into()),
      _ => Ok(())
    }
  }
}

impl std::fmt::Display for ShortcutBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.command().into_iter()
      .map(|str| if str.contains(" ") { format!("\"{}\"", str) } else { str.to_string() })
      .reduce(|a, b| format!("{} {}", a, b)).unwrap()
    )
  }
}

pub struct ShortcutBuilder {
  pub name: Option<String>,
  pub interpreters: Option<Vec<Interpreter>>,
  pub body: Option<ShortcutBody>
}

impl ShortcutBuilder {
  pub fn new() -> Self {
    ShortcutBuilder {
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

  pub fn command_script(
    mut self,
    command: Vec<String>,
    script_path: impl AsRef<path::Path>,
    script_body: impl AsRef<str>,
    script_offset: Option<u8>
  ) -> Result<Self> {
    self.body = Some(ShortcutBody::CommandWithScript{
      cmd: command,
      script: script_path.as_ref().to_path_buf(),
      body: script_body.as_ref().to_string(),
      script_offset
    });
    Ok(self)
  }

  pub fn build(self) -> Shortcut {
    Shortcut {
      name: self.name.unwrap(),
      interpreters: self.interpreters,
      body: self.body.unwrap(),
      startup: None
    }
  }
}