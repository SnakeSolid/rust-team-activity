use reqwest::StatusCode;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub type ActivityStreamsResult<T> = Result<T, ActivityStreamsError>;

#[derive(Debug, Clone)]
pub enum ActivityStreamsError {
    RequestError {
        message: String,
    },
    StatusNotSuccess {
        status_code: u16,
        message: Option<&'static str>,
    },
}

impl ActivityStreamsError {
    pub fn request_error<E>(error: E) -> ActivityStreamsError
    where
        E: Error,
    {
        ActivityStreamsError::RequestError {
            message: format!("{}", error),
        }
    }

    pub fn status_not_success(status: StatusCode) -> ActivityStreamsError {
        ActivityStreamsError::StatusNotSuccess {
            status_code: status.as_u16(),
            message: status.canonical_reason(),
        }
    }
}

impl Display for ActivityStreamsError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ActivityStreamsError::RequestError { ref message } => {
                write!(f, "Request error: `{}`", message)
            }
            ActivityStreamsError::StatusNotSuccess { status_code, .. } => {
                write!(f, "Status code {} not success", status_code)
            }
        }
    }
}

impl Error for ActivityStreamsError {}
