use clap::{Parser, Subcommand};
use crate::{controller::Controller, file::ShortcutFile, errors::Result};

#[derive(Debug, Parser)]
pub struct Cli {
  #[clap(subcommand)]
  pub command: Command
}

#[derive(Debug, Subcommand)]
pub enum Command {
  New {
    name: String,
    #[clap(required = true)]
    command: Vec<String>,
    #[clap(name = "interpreters", short = 'i')]
    override_interpreters: Option<Vec<String>>,
  },
  #[clap(name = "del")]
  Delete {
    #[clap(required = true)]
    names: Vec<String>,
    #[clap(short = 'f')]
    filename: bool
  },
  List {
    #[clap(short = 'e')]
    errors: bool,
    #[clap(short = 'v')]
    verbose: bool
  },
  Make {
    #[clap(name = "interpreters", short = 'i')]
    override_interpreters: Option<Vec<String>>,
    #[clap(required = true)]
    names: Vec<String>
  },
  Clean
}

impl Command {
  pub fn apply(&self, controller: &mut Controller) -> Result<()> {
    match self {
      Self::New { name, command, override_interpreters } =>
        controller.new_link(name, ShortcutFile::builder()
          .name(name)
          .command(command.clone())
          .override_interpreters(override_interpreters.clone())
          .build()
        ),
      Self::Delete { names, filename } =>
        controller.delete(names, *filename),
      Self::List { errors, verbose } =>
        controller.list(*errors, *verbose),
      Self::Make { names, override_interpreters } =>
        controller.make(names, override_interpreters.as_deref()),
      Self::Clean => controller.clean(),
    }
  }
}