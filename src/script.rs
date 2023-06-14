use std::fmt::Display;

use crate::{interpreter::Interpreter, errors::{Result, ScuError}};

#[derive(Clone, Debug)]
pub struct Script<'a> {
  interpreter: &'a Interpreter,
  binary: String,
  args: Vec<String>
}

impl<'a> Script<'a> {
  pub fn new(interpreter: &'a Interpreter, command: Vec<String>) -> Result<Self> {
    command.get(0).ok_or(ScuError::StringError("Expecting at least one element".into())).map(
      |binary| Script {
        interpreter,
        binary: binary.clone(),
        args: command[1..].to_vec()
      }
    )
  }
}

macro_rules! script_display {
  {$($($variant:ident)* => $template:literal [$($options:tt)*])*} => {
    impl Display for Script<'_> {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.interpreter {
          $(
            $(Interpreter::$variant)|* => write!(
              f,
              $template,
              self.binary,
              script_options_display!(self.args, $($options)*)
            ),
          )*
          _ => todo!()
        }
      }
    }    
  };
}

macro_rules! script_options_display {
  ($source:expr, sep $separator:literal wrap $wrapper:literal) => {
    $source.iter().map(|x| format!($wrapper, x)).collect::<Vec<String>>().join($separator)
  };
}

script_display! {
  Bash => "#!/bin/sh
\"{}\" {} \"$@\"
exit $?" [sep " " wrap "\"{}\""]

  Cmd => "@ECHO off
\"{}\" {} %*
EXIT /b %errorlevel%" [sep " " wrap "\"{}\""]

  Python Pythonw => "from subprocess import run
from sys import argv

program = [\"{}\", {}]

code = run(program + argv[1:]).returncode
exit(code)" [sep ", " wrap "\"{}\""]

  Powershell => "& \"{}\" {} $args
exit $LASTEXITCODE" [sep " " wrap "\"{}\""]
}
