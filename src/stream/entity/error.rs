use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub type ObjectResult<T> = Result<T, ObjectError>;

#[derive(Debug)]
pub enum ObjectError {
    MissingObjectType,
    WrongObjectType { object_type: String },
    ElementNotFound { element_name: String },
}

impl ObjectError {
    pub fn wrong_object_type(object_type: &str) -> ObjectError {
        warn!("Wrong object type: {}", object_type);

        ObjectError::WrongObjectType {
            object_type: object_type.into(),
        }
    }

    pub fn element_not_found(element_name: &str) -> ObjectError {
        warn!("Element not found: {}", element_name);

        ObjectError::ElementNotFound {
            element_name: element_name.into(),
        }
    }
}

impl Display for ObjectError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ObjectError::MissingObjectType => write!(f, "Missing object type"),
            ObjectError::WrongObjectType { ref object_type } => {
                write!(f, "Wrong object type: `{}`", object_type)
            }
            ObjectError::ElementNotFound { ref element_name } => {
                write!(f, "Element `{}` not found", element_name)
            }
        }
    }
}

impl Error for ObjectError {}

pub type EntryResult<T> = Result<T, EntryError>;

#[derive(Debug)]
pub enum EntryError {
    ElementNotFound { element_name: String },
    XmlEventError { message: String },
    ReadObjectError { message: String },
}

impl EntryError {
    pub fn element_not_found(element_name: &str) -> EntryError {
        warn!("Element not found: {}", element_name);

        EntryError::ElementNotFound {
            element_name: element_name.into(),
        }
    }

    pub fn xml_event_error<E>(error: E) -> EntryError
    where
        E: Error,
    {
        warn!("XML event error: {}", error);

        EntryError::XmlEventError {
            message: format!("{}", error),
        }
    }

    pub fn read_object_error<E>(error: E) -> EntryError
    where
        E: Error,
    {
        warn!("Read object error: {}", error);

        EntryError::ReadObjectError {
            message: format!("{}", error),
        }
    }
}

impl Display for EntryError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            EntryError::ElementNotFound { ref element_name } => {
                write!(f, "Element `{}` not found", element_name)
            }
            EntryError::XmlEventError { ref message } => write!(f, "XML read error: `{}`", message),
            EntryError::ReadObjectError { ref message } => {
                write!(f, "Read object error: `{}`", message)
            }
        }
    }
}

impl Error for EntryError {}

pub type FeedResult<T> = Result<T, FeedError>;

#[derive(Debug)]
pub enum FeedError {
    ElementNotFound { element_name: String },
    XmlEventError { message: String },
    ReadEntryError { message: String },
}

impl FeedError {
    pub fn element_not_found(element_name: &str) -> FeedError {
        warn!("Element not found: {}", element_name);

        FeedError::ElementNotFound {
            element_name: element_name.into(),
        }
    }

    pub fn xml_event_error<E>(error: E) -> FeedError
    where
        E: Error,
    {
        warn!("XML event error: {}", error);

        FeedError::XmlEventError {
            message: format!("{}", error),
        }
    }

    pub fn read_entry_error<E>(error: E) -> FeedError
    where
        E: Error,
    {
        warn!("Read entry error: {}", error);

        FeedError::ReadEntryError {
            message: format!("{}", error),
        }
    }
}

impl Display for FeedError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            FeedError::ElementNotFound { ref element_name } => {
                write!(f, "Element `{}` not found", element_name)
            }
            FeedError::XmlEventError { ref message } => write!(f, "XML read error: `{}`", message),
            FeedError::ReadEntryError { ref message } => {
                write!(f, "Read entry error: `{}`", message)
            }
        }
    }
}

impl Error for FeedError {}
