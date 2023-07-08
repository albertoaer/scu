mod cli;
mod paths;
mod errors;
mod script;
mod reader;
mod startup;
mod shortcut;
mod controller;
mod interpreter;

use clap::Parser;

fn main() {
  let args = cli::Cli::parse();
  let mut controller = controller::Controller::new().unwrap();

  controller.setup().unwrap();

  let result = args.command.apply(&mut controller);
  controller.handle_result(result);
}
