use std::{fs, path, env};

use crate::{file::ShortcutFile, errors::Result};

pub struct Controller {
  path: path::PathBuf
}

const META_FOLDER: &str = "meta";
const BIN_FOLDER: &str = "bin";
const SUFFIX: &str = ".toml";

impl Controller {
  pub fn new() -> Result<Self> {
    Ok(Controller { path: env::current_exe()?.parent().unwrap().into()  })
  }

  pub fn meta_folder(&self) -> path::PathBuf {
    self.path.as_path().join(META_FOLDER)
  }

  pub fn bin_folder(&self) -> path::PathBuf {
    self.path.as_path().join(BIN_FOLDER)
  }

  pub fn setup(&mut self) -> Result<()> {
    fs::create_dir_all(self.meta_folder())?;
    fs::create_dir_all(self.bin_folder()).map_err(|err| err.into())
  }

  pub fn new_link(&mut self, name: impl AsRef<str>, file: ShortcutFile) -> Result<()> {
    file.store(self.meta_folder().join(format!("{}{}", name.as_ref(), SUFFIX)))
  }

  pub fn delete(&mut self, names: &[impl AsRef<str>], by_filename: bool) -> Result<()> {
    let targets: Vec<&str> = names.into_iter().map(|x| x.as_ref()).collect();
    let filter = if by_filename {
      Box::new(
        |entry: &fs::DirEntry| targets.contains(&entry.file_name().to_str().unwrap())
      ) as Box<dyn Fn(&fs::DirEntry)->bool>
    } else {
      Box::new(|entry: &fs::DirEntry| match ShortcutFile::load(entry.path()) {
        Ok(file) => targets.contains(&file.name.as_str()),
        Err(_) => false,
      }) as Box<dyn Fn(&fs::DirEntry)->bool>
    };
    for entry in fs::read_dir(self.meta_folder())?.into_iter().filter_map(|x| x.ok()).filter(filter) {
      if entry.metadata().map(|m| m.is_file()).unwrap_or(true) {
        fs::remove_file(entry.path())?;
      } else {
        fs::remove_dir(entry.path())?;
      }
    }
    Ok(())
  }

  pub fn list(&self, notify_errors: bool, verbose: bool) -> Result<()> {
    for entry in fs::read_dir(self.meta_folder())?.into_iter().filter_map(|x| x.ok()) {
      match ShortcutFile::load(entry.path()) {
        Ok(file) => println!("> {} => {}", file.name, file.body),
        Err(err) if notify_errors => {
          println!("> Invalid file: {}", entry.file_name().to_str().unwrap());
          if verbose {
            println!("'''\n{}'''", err)
          }
        }
        _  => {}
      }
    }
    Ok(())
  }

  pub fn make(
    &mut self, names: &[impl AsRef<str>], override_interpreters: Option<&[impl AsRef<str>]>
  ) -> Result<()> {
    Ok(())
  }

  pub fn clean(&mut self) -> Result<()> {
    fs::remove_dir_all(self.bin_folder())?;
    fs::create_dir(self.bin_folder()).map_err(|err| err.into())
  }
}