use clap::{Parser, Subcommand};
use crate::{controller::Controller, file::ShortcutFile, errors::Result};

#[derive(Debug, Parser)]
pub struct Cli {
  #[clap(subcommand)]
  pub command: Command
}

#[derive(Debug, Subcommand)]
pub enum Command {
  #[clap(about = "Create a shortcut template, it can overwrite an existing one")]
  New {
    name: String,
    #[arg(required = true)]
    command: Vec<String>,
    #[arg(name = "interpreters", short = 'i', num_args(0..))]
    override_interpreters: Option<Vec<String>>,
  },
  #[clap(name = "del", about = "Delete a shortcut template")]
  Delete {
    #[arg(required = true)]
    names: Vec<String>,
    #[arg(short = 'f')]
    filename: bool
  },
  #[clap(about = "List all the existing resources")]
  List {
    #[arg(short = 'e')]
    errors: bool,
    #[arg(short = 'v')]
    verbose: bool
  },
  Make {
    #[arg(name = "interpreters", short = 'i', num_args(0..))]
    override_interpreters: Option<Vec<String>>,
    #[arg(required = true)]
    names: Vec<String>
  },
  #[clap(about = "Clean all the created binaries")]
  Clean
}

impl Command {
  pub fn apply(&self, controller: &mut Controller) -> Result<()> {
    match self {
      Self::New { name, command, override_interpreters } =>
        controller.new_shortcut(name, ShortcutFile::builder()
          .name(name)
          .command(command.clone())
          //FIXME: .override_interpreters(override_interpreters.clone())
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