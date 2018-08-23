use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub type WorkerResult<T> = Result<T, WorkerError>;

#[derive(Debug)]
pub enum WorkerError {
    DatabaseError { message: String },
    EntityReadError { message: String },
    DateParseError { message: String },
    SerializationError { message: String },
    StreamError { message: String },
}

impl WorkerError {
    pub fn database_error<E>(error: E) -> WorkerError
    where
        E: Error,
    {
        warn!("Database error: {}", error);

        WorkerError::DatabaseError {
            message: format!("{}", error),
        }
    }

    pub fn entity_read_error<E>(error: E) -> WorkerError
    where
        E: Error,
    {
        warn!("Entity read error: {}", error);

        WorkerError::EntityReadError {
            message: format!("{}", error),
        }
    }

    pub fn date_parse_error<E>(error: E) -> WorkerError
    where
        E: Error,
    {
        warn!("Date parse error: {}", error);

        WorkerError::DateParseError {
            message: format!("{}", error),
        }
    }

    pub fn serialization_error<E>(error: E) -> WorkerError
    where
        E: Error,
    {
        warn!("Serialization error: {}", error);

        WorkerError::SerializationError {
            message: format!("{}", error),
        }
    }

    pub fn stream_error<E>(error: E) -> WorkerError
    where
        E: Error,
    {
        warn!("Stream error: {}", error);

        WorkerError::StreamError {
            message: format!("{}", error),
        }
    }
}

impl Display for WorkerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            WorkerError::DatabaseError { ref message } => {
                write!(f, "Database error: `{}`", message)
            }
            WorkerError::EntityReadError { ref message } => {
                write!(f, "Entity read error: `{}`", message)
            }
            WorkerError::DateParseError { ref message } => {
                write!(f, "Date parse error: `{}`", message)
            }
            WorkerError::SerializationError { ref message } => {
                write!(f, "Serialization error: `{}`", message)
            }
            WorkerError::StreamError { ref message } => write!(f, "Stream error: `{}`", message),
        }
    }
}

impl Error for WorkerError {}
