use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Debug)]
pub enum DatabaseError {
    MutexLockError,
    ConnectionError { message: String },
    ExecutionError { message: String },
    PrepareFailed { message: String },
    BindFailed { message: String },
    NextFailed { message: String },
    NoSuchColumn,
    NoSuchValue,
}

macro_rules! error_method {
    ($name:ident, $option:ident, $message:expr) => {
        pub fn $name<E>(error: E) -> DatabaseError
        where
            E: Error,
        {
            warn!("{}: {}", $message, error);

            DatabaseError::$option {
                message: format!("{}", error),
            }
        }
    };
}

impl DatabaseError {
    pub fn mutex_lock_error<E>(error: E) -> DatabaseError
    where
        E: Error,
    {
        warn!("Mutex lock error: {}", error);

        DatabaseError::MutexLockError
    }

    error_method!(connection_error, ConnectionError, "Connection error");
    error_method!(execution_error, ExecutionError, "Execution error");
    error_method!(prepare_failed, PrepareFailed, "Prepare failed");
    error_method!(bind_failed, BindFailed, "Bind failed");
    error_method!(next_failed, NextFailed, "Next failed");

    pub fn no_such_column() -> DatabaseError {
        DatabaseError::NoSuchColumn
    }

    pub fn no_such_value() -> DatabaseError {
        DatabaseError::NoSuchValue
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            DatabaseError::MutexLockError => write!(f, "Mutex lock error"),
            DatabaseError::ConnectionError { ref message } => {
                write!(f, "Connection error: {}", message)
            }
            DatabaseError::ExecutionError { ref message } => {
                write!(f, "Execution error: {}", message)
            }
            DatabaseError::PrepareFailed { ref message } => {
                write!(f, "Prepare failed: {}", message)
            }
            DatabaseError::BindFailed { ref message } => write!(f, "Bind failed: {}", message),
            DatabaseError::NextFailed { ref message } => write!(f, "Next failed: {}", message),
            DatabaseError::NoSuchColumn => write!(f, "No such column"),
            DatabaseError::NoSuchValue => write!(f, "No such value"),
        }
    }
}

impl Error for DatabaseError {}
