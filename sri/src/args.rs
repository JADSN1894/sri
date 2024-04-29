use std::str::FromStr;

use crate::{
  airtifact_path::AirtifactPathSri,
  algorithm::AlgorithmSri,
  constants::MAX_OF_ARGS,
  error::{SriError, SriErrorTypes},
  sri::Sri,
};

pub(super) struct SriArgs;

impl TryFrom<Vec<String>> for SriArgs {
  type Error = SriError;

  fn try_from(args: Vec<String>) -> Result<Self, Self::Error> {
    let args_len = args.len();

    if args_len == 0 {
      return Err(SriErrorTypes::NoArgs.into());
    }

    if args_len >= 1 && args_len < MAX_OF_ARGS {
      return Err(SriErrorTypes::MustContainsTwoArguments.into());
    }

    if args_len == MAX_OF_ARGS {
      let algorithm: AlgorithmSri = FromStr::from_str(&args[0])?;
      let airtifact_path: AirtifactPathSri = (&args[1]).into();
      Sri::try_from((algorithm, airtifact_path))?;
    }

    if args_len > MAX_OF_ARGS {
      return Err(SriErrorTypes::OverflowArgs(args_len.try_into()?).into());
    }

    Ok(SriArgs)
  }
}
