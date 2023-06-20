use std::{io::{self, Read}, fs::{read}, path};

use crate::errors::Result;

pub fn from_stdin() -> Result<String> {
  let mut handle = io::stdin().lock();
  let mut input = Vec::new();
  handle.read_to_end(&mut input);
  String::from_utf8(input).map_err(|err| err.into())
}

pub fn from_file(source: impl AsRef<path::Path>) -> Result<String> {
  read(source).map_err(|err| err.into())
    .and_then(|bytes| String::from_utf8(bytes).map_err(|err| err.into()))
}