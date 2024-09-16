#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    DotenvError(dotenv::Error),
    ConfigurationError(std::env::VarError),
    DatabaseError(tokio_postgres::Error),
    RequestError(reqwest::Error),
    BalanceUnavailable {
        platform: String,
        error: BalanceError,
        json: serde_json::Value,
    },
}

#[derive(Debug)]
pub enum BalanceError {
    MissingField(&'static str),
    ParseError(&'static str),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<dotenv::Error> for Error {
    fn from(error: dotenv::Error) -> Self {
        Error::DotenvError(error)
    }
}

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        Error::ConfigurationError(error)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(error: tokio_postgres::Error) -> Self {
        Error::DatabaseError(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::RequestError(error)
    }
}
