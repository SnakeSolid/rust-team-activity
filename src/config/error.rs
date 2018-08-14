use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug)]
pub enum ConfigError {
    IoError { message: String },
    DeserializationFailed { message: String },
}

impl ConfigError {
    pub fn io_error<E>(error: E) -> ConfigError
    where
        E: Error,
    {
        error!("IO error: {}", error);

        ConfigError::IoError {
            message: format!("{}", error),
        }
    }

    pub fn deserialization_failed<E>(error: E) -> ConfigError
    where
        E: Error,
    {
        error!("Deserialization error: {}", error);

        ConfigError::DeserializationFailed {
            message: format!("{}", error),
        }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ConfigError::IoError { ref message } => write!(f, "IO error: {}", message),
            ConfigError::DeserializationFailed { ref message } => {
                write!(f, "Deserialization failed: {}", message)
            }
        }
    }
}

impl Error for ConfigError {}
