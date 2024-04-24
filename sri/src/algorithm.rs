use std::{
    fmt::{self, Display},
    str::FromStr,
};

use super::error::{SriError, SriErrorTypes, SriResult};

#[derive(Clone, Debug, Default)]
pub(super) enum Algorithm {
    Sha128,
    Sha256,
    #[default]
    Sha512,
    Unknown(String),
}

impl Algorithm {
    fn hexadecimal_to_base64(hexadecimal: &str) -> SriResult<String> {
        use base64::{engine::general_purpose::STANDARD, Engine as _};

        let bytes = (0..hexadecimal.len() / 2)
            .map(|i| u8::from_str_radix(&hexadecimal[2 * i..2 * i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()?;

        let base64_string = STANDARD.encode(&bytes);
        Ok(base64_string)
    }

    pub(super) fn integrity_hash(&self, content: String) -> SriResult<String> {
        let integrity_hash = format!(
            "{}-{}",
            self.to_string().to_lowercase(),
            self.hash(content)?
        );

        Ok(integrity_hash)
    }

    fn hash(&self, data: String) -> SriResult<String> {
        match self {
            Algorithm::Sha128 => {
                use sha1::{Digest, Sha1};

                //* Create a Sha1 object
                let mut hasher = Sha1::new();

                //* Write input message
                hasher.update(data.as_bytes());

                //* Read hash digest and consume hasher
                let hash = hasher.finalize();
                let hash_hexadecimal: String =
                    hash.iter().map(|byte| format!("{:02x}", byte)).collect();

                //* Convert hexadecimal hash to base64
                let ouptut = Self::hexadecimal_to_base64(&hash_hexadecimal)?;

                Ok(ouptut)
            }
            Algorithm::Sha256 => {
                use sha2::{Digest, Sha256};

                //* Create a Sha256 object
                let mut hasher = Sha256::new();

                //* Write input message
                hasher.update(data.as_bytes());

                //* Read hash digest and consume hasher
                let hash = hasher.finalize();
                let hash_hexadecimal: String =
                    hash.iter().map(|byte| format!("{:02x}", byte)).collect();

                //* Convert hexadecimal hash to base64
                let ouptut = Self::hexadecimal_to_base64(&hash_hexadecimal)?;

                Ok(ouptut)
            }
            Algorithm::Sha512 => {
                use sha2::{Digest, Sha512};

                //* Create a Sha512 object
                let mut hasher = Sha512::new();

                //* Write input message
                hasher.update(data.as_bytes());

                //* Read hash digest and consume hasher
                let hash = hasher.finalize();
                let hash_hexadecimal: String =
                    hash.iter().map(|byte| format!("{:02x}", byte)).collect();

                //* Convert hexadecimal hash to base64
                let ouptut = Self::hexadecimal_to_base64(&hash_hexadecimal)?;

                Ok(ouptut)
            }
            Algorithm::Unknown(invalid_algorithm) => {
                Err(SriErrorTypes::InvalidAlgorithm(invalid_algorithm.to_string().into()).into())
            }
        }
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sha128 => write!(f, "Sha128"),
            Self::Sha256 => write!(f, "Sha256"),
            Self::Sha512 => write!(f, "Sha512"),
            Self::Unknown(unknown_algorithm) => write!(f, "{unknown_algorithm}"),
        }
    }
}

impl FromStr for Algorithm {
    type Err = SriError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();
        match value {
            "Sha128" => Ok(Self::Sha128),
            "Sha256" => Ok(Self::Sha256),
            "Sha512" => Ok(Self::Sha512),
            invalid_algorithm => {
                Err(SriErrorTypes::InvalidAlgorithm(invalid_algorithm.to_string().into()).into())
            }
        }
    }
}

impl From<String> for Algorithm {
    fn from(value: String) -> Self {
        let value = value.trim();
        match value {
            "Sha128" => Self::Sha128,
            "Sha256" => Self::Sha256,
            "Sha512" => Self::Sha512,
            invalid_algorithm => Self::Unknown(invalid_algorithm.to_string()),
        }
    }
}
