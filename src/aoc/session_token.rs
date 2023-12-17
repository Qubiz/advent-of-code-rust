use std::error::Error;
use std::str::FromStr;

#[derive(Debug)]
pub enum SessionTokenError {
    InvalidLength { expected: usize, actual: usize },
}

impl std::fmt::Display for SessionTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionTokenError::InvalidLength { expected, actual } => {
                write!(f, "invalid session token length, expected {}, got {}", expected, actual)
            }
        }
    }
}

impl Error for SessionTokenError {}

pub struct SessionToken(pub(crate) String);

impl FromStr for SessionToken {
    type Err = SessionTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const EXPECTED_LENGTH: usize = 128;
        if s.len() != EXPECTED_LENGTH {
            return Err(SessionTokenError::InvalidLength {
                expected: EXPECTED_LENGTH,
                actual: s.len(),
            });
        }
        Ok(SessionToken(String::from(s)))
    }
}

pub fn from_env(key: &str) -> Result<SessionToken, Box<dyn Error>> {
    std::env::var(key)
        .map_err(|error| format!("Failed to read session token from environment variable {}: {}", key, error))?
        .parse::<SessionToken>()
        .map_err(|error| format!("Failed to parse session token: {}", error).into())
}