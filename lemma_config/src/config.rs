use std::{env, fmt::Display};

use crate::log::LogLevel;

#[derive(Debug)]
pub struct Config {
    pub log_level: LogLevel,
}

const VAR_LOG_LEVEL: &'static str = "LOG_LEVEL";

impl Config {
    pub fn from_env() -> Result<Self, InvalidConfigError> {
        let log_level = env::var(VAR_LOG_LEVEL)
            .map_err(|_| InvalidConfigError::MissingVariable(VAR_LOG_LEVEL.to_string()))?;
        let log_level = LogLevel::from_str(&log_level).ok_or(
            InvalidConfigError::InvalidVariable(VAR_LOG_LEVEL.to_string()),
        )?;

        Ok(Self { log_level })
    }
}

#[derive(Debug)]
pub enum InvalidConfigError {
    MissingVariable(String),
    InvalidVariable(String),
}

impl Display for InvalidConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidConfigError::MissingVariable(var) => {
                write!(f, "invalid configuration - missing variable: {}", var)
            }
            InvalidConfigError::InvalidVariable(var) => {
                write!(f, "invalid configuration - invalid variable: {}", var)
            }
        }
    }
}

impl std::error::Error for InvalidConfigError {}
