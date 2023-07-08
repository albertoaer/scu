use std::{fs, path, env, fmt, borrow::Borrow};

use crate::{
  shortcut::{Shortcut, ShortcutFile, self},
  errors::{Result, ScuError},
  interpreter::Interpreter,
  startup::{StartupReference, self}
};

pub struct Controller {
  path: path::PathBuf
}

const BASE_DIR: &str = "scu_data";
const META_DIR: &str = "meta";
const BIN_DIR: &str = "bin";
const RES_DIR: &str = "res";
const SUFFIX: &str = ".toml";

impl Controller {
  pub fn new() -> Result<Self> {
    Ok(Controller { path: env::current_exe()?.parent().unwrap().into() })
  }

  pub fn meta_dir(&self) -> path::PathBuf {
    self.path.as_path().join(BASE_DIR).join(META_DIR)
  }

  pub fn bin_dir(&self) -> path::PathBuf {
    self.path.as_path().join(BASE_DIR).join(BIN_DIR)
  }

  pub fn res_dir(&self) -> path::PathBuf {
    self.path.as_path().join(BASE_DIR).join(RES_DIR)
  }

  pub fn create_resource(&self, file: impl AsRef<path::Path>) -> Result<path::PathBuf> {
    Ok(self.res_dir().join(file.as_ref().file_name().ok_or(ScuError::StringError("Unable to create resource".into()))?))
  }

  pub fn setup(&mut self) -> Result<()> {
    fs::create_dir_all(self.meta_dir())?;
    fs::create_dir_all(self.bin_dir())?;
    fs::create_dir_all(self.res_dir()).map_err(|err| err.into())
  }

  pub fn new_shortcut_file(&mut self, name: impl AsRef<str>, file: Shortcut) -> ShortcutFile {
    ShortcutFile::new(file, self.meta_dir().join(format!("{}{}", name.as_ref(), SUFFIX)))
  }

  pub fn delete(&mut self, names: &[impl AsRef<str>], by_filename: bool) -> Result<()> {
    let targets: Vec<&str> = names.into_iter().map(|x| x.as_ref()).collect();
    let filter: Box<dyn Fn(&fs::DirEntry)->bool> = if by_filename {
      Box::new(
        |entry| targets.contains(&entry.file_name().to_str().unwrap())
      )
    } else {
      Box::new(
        |entry| ShortcutFile::load(entry.path()).map(|file| targets.contains(&file.name.as_str())).unwrap_or(false)
      )
    };
    for entry in fs::read_dir(self.meta_dir())?.into_iter().filter_map(|x| x.ok()).filter(filter) {
      if entry.metadata().map(|m| m.is_file()).unwrap_or(true) {
        fs::remove_file(entry.path())?;
      } else {
        fs::remove_dir(entry.path())?;
      }
    }
    Ok(())
  }

  pub fn get_all(&self) -> Result<impl Iterator<Item = (fs::DirEntry, Result<ShortcutFile>)>> {
    Ok(fs::read_dir(self.meta_dir())?.into_iter().filter_map(|x| x.ok()).map(|entry| {
      let path = entry.path();
      (entry, ShortcutFile::load(path))
    }))
  }

  pub fn list(&self, notify_errors: bool, verbose: bool) -> Result<()> {
    Ok(for (entry, shortcut) in self.get_all()? {
      match shortcut {
        Ok(shortcut) => {
          println!("> {} => {}", shortcut.name, shortcut.body);
          if verbose {
            if let Some(interpreters) = &shortcut.interpreters {
              println!(
                " |> Interpreters: {}",
                interpreters.iter().map(|i| i.name().to_string())
                  .reduce(|a, b| format!("{}, {}", a, b)).unwrap_or(String::new())
              )
            }
            if let Some(startup) = &shortcut.startup {
              println!(" |> Startup: {}", startup);
            }
          }
        },
        Err(err) if notify_errors => {
          println!("> Invalid file: {}", entry.file_name().to_str().unwrap());
          if verbose {
            println!("'''\n{}'''", err);
          }
        }
        _ => {}
      }
    })
  }

  pub fn find_shortcut(&self, name: impl AsRef<str>) -> Result<ShortcutFile> {
    ShortcutFile::load(self.meta_dir().join(format!("{}{}", name.as_ref(), SUFFIX)))
  }

  pub fn find_shortcuts(&self, names: &[impl AsRef<str>]) -> Result<Vec<ShortcutFile>> {
    names.iter().map(|name| self.find_shortcut(name)).collect::<Result<_>>()
  }

  pub fn make(&mut self, shortcut: &ShortcutFile, interpreters: Option<&[impl AsRef<str>]>) -> Result<()> {
    let collected_interpreters = Interpreter::try_collect(interpreters)?;
    let all_interpreters = Interpreter::all();
    let interpreters = [
      collected_interpreters.as_deref(),
      shortcut.interpreters.as_deref(),
      Some(all_interpreters.as_slice())
    ].into_iter().find(|x| x.is_some()).unwrap().unwrap();
    shortcut.write_resources()?;
    Ok(for interpreter in interpreters {
      let script = shortcut.script(interpreter)?;
      fs::write(
        self.bin_dir().join(format!(
          "{}{}",
          shortcut.name,
          if interpreter.prefer_no_extension() { "" } else { interpreter.extension() }
        )),
        format!("{}", script)
      )?;
    })
  }

  pub fn startup_set(&mut self, shortcut: &mut ShortcutFile, force: bool) -> Result<bool> {
    if shortcut.startup.is_none() || force {
      let startup = StartupReference::create(shortcut)?;
      shortcut.update_startup_reference(Some(startup));
      return shortcut.store().map(|_| true)
    }
    Ok(false)
  }
  
  pub fn startup_quit(&mut self, shortcut: &mut ShortcutFile) -> Result<bool> {
    if let Some(startup) = &shortcut.startup {
      startup.delete()?;
      shortcut.update_startup_reference(None);
      return shortcut.store().map(|_| true)
    }
    Ok(false)
  }

  pub fn clean(&mut self) -> Result<()> {
    fs::remove_dir_all(self.bin_dir())?;
    fs::create_dir(self.bin_dir())?;
    fs::remove_dir_all(self.res_dir())?;
    fs::create_dir(self.res_dir()).map_err(|err| err.into())
  }

  pub fn notify_changes(&self, verb: impl fmt::Display, count: i32) {
    println!("{} {} shortcut{}", verb, count, if count == 1 { "" } else { "s" })
  }

  pub fn handle_error(&self, err: impl Borrow<ScuError>) {
    println!("Got error: {}", err.borrow())
  }

  pub fn handle_result<T>(&self, result: impl Borrow<Result<T>>) {
    if let Err(err) = result.borrow() {
      self.handle_error(err);
    }
  }

  pub fn log(&self, data: impl fmt::Display) {
    println!("{}", data)
  }

  pub fn operate_many<'a, T>(&mut self, items: &'a mut [T], mut action: impl FnMut(&mut Controller, &'a mut T) -> Result<bool>) -> i32 {
    items.into_iter().map(|item| action(self, item).map_err(|err| {
      self.handle_error(&err);
      err
    }).unwrap_or(false) as i32).sum()
  }
}