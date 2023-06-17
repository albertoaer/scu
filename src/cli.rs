use clap::{Parser, Subcommand};
use crate::{controller::Controller, shortcut::Shortcut, errors::Result, interpreter::Interpreter};

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
  #[clap(about = "Generate executable scripts for the desired interpreters")]
  Make {
    #[arg(short, num_args(0..))]
    interpreters: Option<Vec<String>>,
    #[arg(required = false)]
    names: Vec<String>,
    #[arg(short)]
    all: bool,
    #[arg(short)]
    clean: bool
  },
  #[clap(about = "Clean all the created binaries")]
  Clean,
  #[clap(about = "Returns the binaries directory")]
  Bin
}

impl Command {
  pub fn apply(&self, controller: &mut Controller) -> Result<()> {
    match self {
      Self::New { name, command, interpreters, make } => {
        let shortcut = Shortcut::builder()
          .name(name)
          .command(command.clone())
          .interpreters(Interpreter::try_collect(interpreters.as_deref())?)
          .build();
        controller.new_shortcut(name, &shortcut)?;
        if *make {
          controller.make(&[shortcut], None::<&[&str]>).map(drop)
        } else {
          Ok(())
        }
      }
      Self::Delete { names, filename } =>
        controller.delete(names, *filename),
      Self::List { errors, verbose } =>
        controller.list(*errors, *verbose),
      Self::Make { names, interpreters, all, clean } => {
        let shortcuts = if *all {
          controller.get_all()?.filter_map(|(_, result)| result.ok()).collect()
        } else {
          names.iter().map(|name| controller.find_shortcut(name)).collect::<Result<Vec<Shortcut>>>()?
        };
        if *clean {
          controller.clean()?;
        }
        controller.make(&shortcuts, interpreters.as_deref()).map(
          |count| println!("Made {} shortcut{}", count, if count == 1 { "" } else { "s" })
        )
      },
      Self::Clean => controller.clean(),
      Self::Bin => Ok(
        println!("{}", controller.bin_dir().to_string_lossy().replace("\\\\?\\", "").replace("\\\\", "\\"))
      ),
    }
  }
}