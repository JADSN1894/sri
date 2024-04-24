use std::{error::Error as StdError, io::Error as StdIoErro, num::ParseIntError, rc::Rc};
use std::{
    fmt::{self, Display},
    num::TryFromIntError,
};

use super::{algorithm::Algorithm, constants::MAX_OF_ARGS};

pub(super) type SriResult<T> = Result<T, SriError>;

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub(super) enum SriErrorTypes {
    #[default]
    Unknown,
    NoArgs,
    OverflowArgs(u8),
    MustContainsTwoArguments,
    InvalidAlgorithm(Algorithm),
    ParseIntError(ParseIntError),
    TryFromIntError(TryFromIntError),
    StdIoError(Rc<StdIoErro>),
    ImpossibleState,
}

pub(super) struct SriError {
    code: SriErrorCode,
    error: SriErrorTypes,
}

impl Display for SriErrorTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::NoArgs => write!(f, "Must contains at least two arguments"),
            Self::OverflowArgs(qtde_args) => {
                write!(
                    f,
                    "The limit of arguments is {MAX_OF_ARGS}. You provide '{qtde_args}'"
                )
            }
            Self::MustContainsTwoArguments => {
                write!(f, "Must contains at least {MAX_OF_ARGS} arguments")
            }

            Self::ParseIntError(error) => write!(f, "{error}"),
            Self::TryFromIntError(error) => write!(f, "{error}"),
            Self::StdIoError(error) => write!(f, "{error}"),
            Self::InvalidAlgorithm(algorithm) => {
                write!(f, "The '{algorithm}' is not valid algorithm")
            }

            Self::ImpossibleState => write!(f, "Impossible state"),
        }
    }
}

#[derive(Debug)]
pub(super) struct SriErrorCode(u8);

impl SriErrorCode {
    fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for SriErrorCode {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Display for SriErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.get();
        let code_fmt = format!("{:0>4}", code.to_string());
        write!(f, "{code_fmt}")
    }
}

impl From<SriErrorTypes> for SriError {
    fn from(value: SriErrorTypes) -> Self {
        Self {
            code: value.clone().into(),
            error: value,
        }
    }
}

impl From<SriErrorTypes> for SriErrorCode {
    fn from(value: SriErrorTypes) -> Self {
        match value {
            SriErrorTypes::Unknown => u8::MIN.into(),
            SriErrorTypes::NoArgs => 1.into(),
            SriErrorTypes::OverflowArgs(_) => 2.into(),
            SriErrorTypes::MustContainsTwoArguments => 3.into(),
            SriErrorTypes::InvalidAlgorithm(_) => 4.into(),
            SriErrorTypes::ParseIntError(_) => 5.into(),
            SriErrorTypes::TryFromIntError(_) => 6.into(),
            SriErrorTypes::StdIoError(_) => 7.into(),
            SriErrorTypes::ImpossibleState => u8::MAX.into(),
        }
    }
}

impl fmt::Debug for SriError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self { code, error } = self;

        write!(f, "[{code}]: {error}")
    }
}

impl fmt::Display for SriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { code, error } = self;

        write!(f, "[{code}]: {error}")
    }
}

impl StdError for SriError {}

impl From<ParseIntError> for SriError {
    fn from(value: ParseIntError) -> Self {
        SriErrorTypes::ParseIntError(value.into()).into()
    }
}

impl From<TryFromIntError> for SriError {
    fn from(value: TryFromIntError) -> Self {
        SriErrorTypes::TryFromIntError(value).into()
    }
}

impl From<StdIoErro> for SriError {
    fn from(value: StdIoErro) -> Self {
        SriErrorTypes::StdIoError(value.into()).into()
    }
}
