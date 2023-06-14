use serde::{Serialize, Deserialize};

use crate::errors::ScuError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Interpreter {
  Bash,
  Cmd,
  Python,
  Pythonw,
  Powershell
}

impl Interpreter {
  pub fn all() -> [Self; 5] {
    [
      Self::Bash,
      Self::Cmd,
      Self::Python,
      Self::Pythonw,
      Self::Powershell
    ]
  }

  pub fn from_name(name: impl AsRef<str>) -> Option<Self> {
    match name.as_ref().to_ascii_lowercase().as_str() {
      "bash" => Some(Self::Bash),
      "cmd" | "batch" => Some(Self::Cmd),
      "python" => Some(Self::Python),
      "pythonw" => Some(Self::Pythonw),
      "powershell" => Some(Self::Powershell),
      _ => None
    }
  }

  pub fn from_extension(extension: impl AsRef<str>) -> Option<Self> {
    match extension.as_ref().to_ascii_lowercase().as_str() {
      ".sh" => Some(Self::Bash),
      ".bat" => Some(Self::Cmd),
      ".py" => Some(Self::Python),
      ".pyw" => Some(Self::Pythonw),
      ".ps1" => Some(Self::Powershell),
      _ => None
    }
  }

  pub fn name(&self) -> &'static str {
    match self {
      Self::Bash => "bash",
      Self::Cmd => "cmd",
      Self::Python => "python",
      Self::Pythonw => "pythonw",
      Self::Powershell => "powershell"
    }
  }

  pub fn extension(&self) -> &'static str {
    match self {
      Self::Bash => ".sh",
      Self::Cmd => ".bat",
      Self::Python => ".py",
      Self::Pythonw => ".pyw",
      Self::Powershell => ".ps1"
    }
  }
}

impl TryFrom<&str> for Interpreter {
  type Error = ScuError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Self::from_name(value).ok_or(ScuError::StringError(format!("Interpreter not registered {}", value)))
  }
}