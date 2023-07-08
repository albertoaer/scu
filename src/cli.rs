use clap::{Parser, Subcommand};

use crate::{controller::Controller, shortcut::{Shortcut, ShortcutBuilder, ShortcutFile}, errors::Result, interpreter::Interpreter, reader, paths};

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
    args: Vec<String>,
    #[arg(short)]
    source: Option<String>,
    #[arg(short = 'o')]
    arg_offset: Option<u8>,
    #[arg(short)]
    file: bool,
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
  #[clap(about = "List all the existing resources", alias = "ls")]
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
  Bin,
  #[clap(about = "Admin the startup configuration depending on the system")]
  Startup {
    #[arg(required = false)]
    names: Vec<String>,
    #[arg(short)]
    quit: bool,
    #[arg(short)]
    force: bool,
  },
}

fn base_shortcut(name: &String, interpreters: &Option<Vec<String>>) -> Result<ShortcutBuilder> {
  Ok(
    Shortcut::builder()
    .name(name)
    .interpreters(Interpreter::try_collect(interpreters.as_deref())?)
  )
}

impl Command {
  pub fn apply(&self, controller: &mut Controller) -> Result<()> {
    match self {
      Self::New { name, args, source, arg_offset, file, interpreters, make } => {
        let shortcut = controller.new_shortcut_file(name, match source {
          Some(source) => {
            let base = base_shortcut(name, interpreters)?;
            let resource = controller.create_resource(source)?;
            let body = if *file { reader::from_file(source) } else { reader::from_stdin() }?;
            base.command_script(args.clone(), resource, body, *arg_offset)?
          },
          None => base_shortcut(name, interpreters)?.command(args.clone()),
        }.build());
        shortcut.store()?;
        if *make {
          controller.make(&shortcut, None::<&[&str]>)?;
        }
        Ok(())
      },
      Self::Delete { names, filename } =>
        controller.delete(names, *filename),
      Self::List { errors, verbose } =>
        controller.list(*errors, *verbose),
      Self::Make { names, interpreters, all, clean } => {
        let mut shortcuts = if *all {
          controller.get_all()?.filter_map(|(_, result)| result.ok()).collect()
        } else {
          controller.find_shortcuts(names)?
        };
        if *clean {
          controller.clean_dirs()?;
        }
        let action = |controller: &mut Controller, shortcut: &mut _|
          controller.make(shortcut, interpreters.as_deref()).map(|_| true);
        let count = controller.operate_many(&mut shortcuts, action);
        controller.notify_changes("Made", count);
        Ok(())
      },
      Self::Clean => controller.clean_dirs(),
      Self::Bin => Ok(controller.log(paths::stringify_default(controller.bin_dir()))),
      Self::Startup { names, quit, force } => {
        let mut shortcuts = controller.find_shortcuts(names)?;
        let (action, verb) : (Box<dyn FnMut(&mut Controller, &mut ShortcutFile) -> Result<bool>>, _)= if !*quit {
          (Box::new(|controller, shortcut| controller.startup_set(shortcut, *force)), "Set")
        } else {
          (Box::new(|controller, shortcut| controller.startup_quit(shortcut)), "Quit")
        };
        let count = controller.operate_many(&mut shortcuts, action);
        controller.notify_changes(verb, count);
        Ok(())
      }
    }
  }
}