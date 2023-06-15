mod cli;
mod errors;
mod script;
mod shortcut;
mod controller;
mod interpreter;

use clap::Parser;

fn main() -> errors::Result<()> {
  let args = cli::Cli::parse();
  let mut controller = controller::Controller::new().unwrap();

  controller.setup().unwrap();

  args.command.apply(&mut controller)
}
