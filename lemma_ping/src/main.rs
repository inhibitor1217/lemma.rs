use lambda_http::{run, service_fn, Body, Error, Request, Response};
use std::{env, fmt::Display};
use tracing::info;

const VAR_LOG_LEVEL: &str = "LOG_LEVEL";

#[derive(Debug)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

struct Config {
    pub log_level: LogLevel,
}

#[derive(Debug)]
enum InvalidConfigError {
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

impl Config {
    pub fn from_env() -> Result<Self, InvalidConfigError> {
        let log_level = env::var(VAR_LOG_LEVEL)
            .map_err(|_| InvalidConfigError::MissingVariable(VAR_LOG_LEVEL.to_string()))?;
        let log_level = LogLevel::from_str(&log_level)
            .map_err(|_| InvalidConfigError::InvalidVariable(VAR_LOG_LEVEL.to_string()))?;

        Ok(Self { log_level })
    }
}

async fn ping(event: Request) -> Result<Response<Body>, Error> {
    info!("{} {}", event.method(), event.uri());

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello AWS Lambda HTTP request".into())
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::from_env()?;

    let log_level = match config.log_level {
        LogLevel::Debug => tracing::Level::DEBUG,
        LogLevel::Info => tracing::Level::INFO,
        LogLevel::Warn => tracing::Level::WARN,
        LogLevel::Error => tracing::Level::ERROR,
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    run(service_fn(ping)).await
}
