mod airtifact_path;
mod algorithm;
mod args;
mod constants;
mod error;
mod sri;
mod utils;

use std::{env, process::ExitCode};

use args::SriArgs;
use error::SriResult;

fn main() -> SriResult<ExitCode> {
  let args = env::args().skip(1).collect::<Vec<String>>();
  SriArgs::try_from(args)?;

  println!("Operation completed successfully.");
  Ok(ExitCode::SUCCESS)
}
