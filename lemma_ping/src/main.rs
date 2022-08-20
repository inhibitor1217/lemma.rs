use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, Response};
use lemma_config::{Config, LogLevel};
use tracing::info;

async fn ping(event: Request) -> Result<Response<Body>, Error> {
    info!("{} {}", event.method(), event.uri().path());

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/plain")
        .body(Body::from(()))
        .map_err(Box::new)?;

    Ok(resp)
}

fn setup_logger(config: &Config) {
    let log_level = match config.log_level {
        LogLevel::Debug => tracing::Level::DEBUG,
        LogLevel::Info => tracing::Level::INFO,
        LogLevel::Warn => tracing::Level::WARN,
        LogLevel::Error => tracing::Level::ERROR,
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .without_time()
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::from_env()?;

    setup_logger(&config);

    run(service_fn(ping)).await
}
