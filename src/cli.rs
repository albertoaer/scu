use clap::{Parser, Subcommand};
use crate::{controller::Controller, file::ShortcutFile, errors::Result, interpreter::Interpreter};

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
    #[arg(short, num_args(0..))]
    interpreters: Option<Vec<String>>,
    #[arg(short, long, default_value_t = false)]
    make: bool
  },
  #[clap(name = "del", about = "Delete a shortcut template")]
  Delete {
    #[arg(required = true)]
    names: Vec<String>,
    #[arg(short)]
    filename: bool
  },
  #[clap(about = "List all the existing resources")]
  List {
    #[arg(short, long)]
    errors: bool,
    #[arg(short, long)]
    verbose: bool
  },
  Make {
    #[arg(short, num_args(0..))]
    interpreters: Option<Vec<String>>,
    #[arg(required = true)]
    names: Vec<String>
  },
  #[clap(about = "Clean all the created binaries")]
  Clean
}

impl Command {
  pub fn apply(&self, controller: &mut Controller) -> Result<()> {
    match self {
      Self::New { name, command, interpreters, make } =>
        controller.new_shortcut(name, ShortcutFile::builder()
          .name(name)
          .command(command.clone())
          .interpreters(Interpreter::try_collect(interpreters.as_deref())?)
          .build()
        ),
      Self::Delete { names, filename } =>
        controller.delete(names, *filename),
      Self::List { errors, verbose } =>
        controller.list(*errors, *verbose),
      Self::Make { names, interpreters } =>
        controller.make(names, interpreters.as_deref()),
      Self::Clean => controller.clean(),
    }
  }
}