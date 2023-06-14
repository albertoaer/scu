mod cli;
mod file;
mod controller;

use std::io;

use clap::Parser;

fn main() -> io::Result<()> {
  let args = cli::Cli::parse();
  let mut controller = controller::Controller::new().unwrap();

  controller.setup().unwrap();

  args.command.apply(&mut controller)
}
