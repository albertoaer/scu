use std::{fs, path, env};

use crate::{shortcut::Shortcut, errors::{Result, ScuError}, interpreter::Interpreter, script::Script};

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
    Ok(Controller { path: env::current_exe()?.parent().unwrap().into()  })
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

  pub fn new_shortcut(&mut self, name: impl AsRef<str>, file: &Shortcut) -> Result<()> {
    file.store(self.meta_dir().join(format!("{}{}", name.as_ref(), SUFFIX)))
  }

  pub fn delete(&mut self, names: &[impl AsRef<str>], by_filename: bool) -> Result<()> {
    let targets: Vec<&str> = names.into_iter().map(|x| x.as_ref()).collect();
    let filter = if by_filename {
      Box::new(
        |entry: &fs::DirEntry| targets.contains(&entry.file_name().to_str().unwrap())
      ) as Box<dyn Fn(&fs::DirEntry)->bool>
    } else {
      Box::new(|entry: &fs::DirEntry| match Shortcut::load(entry.path()) {
        Ok(file) => targets.contains(&file.name.as_str()),
        Err(_) => false,
      }) as Box<dyn Fn(&fs::DirEntry)->bool>
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

  pub fn get_all(&self) -> Result<impl Iterator<Item = (fs::DirEntry, Result<Shortcut>)>> {
    Ok(fs::read_dir(self.meta_dir())?.into_iter().filter_map(|x| x.ok()).map(|entry| {
      let path = entry.path();
      (entry, Shortcut::load(path))
    }))
  }

  pub fn list(&self, notify_errors: bool, verbose: bool) -> Result<()> {
    Ok(for (entry, shortcut) in self.get_all()? {
      match shortcut {
        Ok(file) => println!("> {} => {}", file.name, file.body),
        Err(err) if notify_errors => {
          println!("> Invalid file: {}", entry.file_name().to_str().unwrap());
          if verbose {
            println!("'''\n{}'''", err)
          }
        }
        _ => {}
      }
    })
  }

  pub fn find_shortcut(&self, name: impl AsRef<str>) -> Result<Shortcut> {
    Shortcut::load(self.meta_dir().join(format!("{}{}", name.as_ref(), SUFFIX)))
  }

  pub fn make(
    &mut self, shortcuts: &[Shortcut], interpreters: Option<&[impl AsRef<str>]>
  ) -> Result<i32> {
    let interpreters: Option<Vec<Interpreter>> = Interpreter::try_collect(interpreters)?;
    let all_interpreters = Interpreter::all();
    let mut count = 0;
    for file in shortcuts {
      count += 1;
      let interpreters = [
        interpreters.as_deref(),
        file.interpreters.as_deref(),
        Some(all_interpreters.as_slice())
      ].into_iter().find(|x| x.is_some()).unwrap().unwrap();
      file.write_resources()?;
      for interpreter in interpreters {
        let script = Script::new(interpreter, file.command())?;
        fs::write(
          self.bin_dir().join(format!(
            "{}{}",
            file.name,
            if interpreter.prefer_no_extension() { "" } else { interpreter.extension() }
          )),
          format!("{}", script)
        )?;
      }
    }
    Ok(count)
  }

  pub fn clean(&mut self) -> Result<()> {
    fs::remove_dir_all(self.bin_dir())?;
    fs::create_dir(self.bin_dir())?;
    fs::remove_dir_all(self.res_dir())?;
    fs::create_dir(self.res_dir()).map_err(|err| err.into())
  }
}