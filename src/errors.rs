use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: port_scanner <example.com>")]
    CliUsage,
}
