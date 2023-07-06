#[cfg(target_os = "windows")]
mod windows_startup {
  use std::{path, fs};

  use serde::{Serialize, Deserialize};

  use home::home_dir;

  use crate::{shortcut::Shortcut, errors::{ScuError, Result}, interpreter::Interpreter};
  
  #[derive(Clone, Debug, Serialize, Deserialize)]
  pub struct StartupReference(path::PathBuf);

  const WIN_PATH: &'static str = "AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup";

  impl StartupReference {
    pub fn create(shortcut: &Shortcut) -> Result<Self> {
      let interpreter = Interpreter::Cmd;
      let path = home_dir().ok_or(ScuError::StringError("".into()))?.join(WIN_PATH)
        .join(format!("{}{}", shortcut.name, interpreter.extension()));
      fs::write(&path, format!("{}", shortcut.script(&interpreter)?))?;
      Ok(StartupReference(path))
    }

    pub fn delete(&self) -> Result<()> {
      fs::remove_file(&self.0).map_err(|err| err.into())
    }
  }
}

#[cfg(target_os = "windows")]
pub use windows_startup::*;