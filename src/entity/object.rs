use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use super::ObjectError;
use super::ObjectResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Object {
    Comment {
        id: String,
        alternate: String,
    },
    File {
        id: String,
        title: String,
        alternate: String,
    },
    Person {
        name: String,
        email: String,
        uri: String,
        photo: String,
        username: String,
    },
    Changeset {
        id: String,
        title: String,
        alternate: String,
    },
    Issue {
        id: String,
        title: String,
        summary: String,
        alternate: String,
    },
    Repository {
        id: String,
        title: String,
        alternate: String,
    },
    Review {
        id: String,
        title: String,
        summary: String,
        alternate: String,
    },
    Page {
        id: String,
        title: String,
        alternate: String,
    },
    Space {
        id: String,
        title: String,
        alternate: String,
    },
}

impl Object {
    pub fn try_from(properties: &HashMap<String, String>) -> ObjectResult<Object> {
        debug!("parsing object from {:?}", properties);

        if let Some(object_type) = properties.get("object-type") {
            if object_type == "http://activitystrea.ms/schema/1.0/comment" {
                Object::parse_comment(properties)
            } else if object_type == "http://activitystrea.ms/schema/1.0/file" {
                Object::parse_file(properties)
            } else if object_type == "http://activitystrea.ms/schema/1.0/person" {
                Object::parse_person(properties)
            } else if object_type == "http://streams.atlassian.com/syndication/types/changeset" {
                Object::parse_changeset(properties)
            } else if object_type == "http://streams.atlassian.com/syndication/types/issue" {
                Object::parse_issue(properties)
            } else if object_type == "http://streams.atlassian.com/syndication/types/repository" {
                Object::parse_repository(properties)
            } else if object_type == "http://streams.atlassian.com/syndication/types/review" {
                Object::parse_review(properties)
            } else if object_type == "http://streams.atlassian.com/syndication/types/page" {
                Object::parse_page(properties)
            } else if object_type == "http://streams.atlassian.com/syndication/types/space" {
                Object::parse_space(properties)
            } else {
                Err(ObjectError::wrong_object_type(object_type))
            }
        } else {
            Err(ObjectError::MissingObjectType)
        }
    }

    fn parse_comment(properties: &HashMap<String, String>) -> ObjectResult<Object> {
        let id = properties
            .get("id")
            .ok_or_else(|| ObjectError::element_not_found("id"))?;
        let alternate = properties
            .get("alternate")
            .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

        Ok(Object::Comment {
            id: id.clone(),
            alternate: alternate.clone(),
        })
    }

    fn parse_file(properties: &HashMap<String, String>) -> ObjectResult<Object> {
        let id = properties
            .get("id")
            .ok_or_else(|| ObjectError::element_not_found("id"))?;
        let title = properties
            .get("title")
            .ok_or_else(|| ObjectError::element_not_found("title"))?;
        let alternate = properties
            .get("alternate")
            .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

        Ok(Object::File {
            id: id.clone(),
            title: title.clone(),
            alternate: alternate.clone(),
        })
    }

    fn parse_person(properties: &HashMap<String, String>) -> ObjectResult<Object> {
        let name = properties
            .get("name")
            .ok_or_else(|| ObjectError::element_not_found("name"))?;
        let email = properties
            .get("email")
            .ok_or_else(|| ObjectError::element_not_found("email"))?;
        let uri = properties
            .get("uri")
            .ok_or_else(|| ObjectError::element_not_found("uri"))?;
        let photo = properties
            .get("photo")
            .ok_or_else(|| ObjectError::element_not_found("photo"))?;
        let username = properties
            .get("username")
            .ok_or_else(|| ObjectError::element_not_found("username"))?;

        Ok(Object::Person {
            name: name.clone(),
            email: email.clone(),
            uri: uri.clone(),
            photo: photo.clone(),
            username: username.clone(),
        })
    }

    fn parse_changeset(properties: &HashMap<String, String>) -> ObjectResult<Object> {
        let id = properties
            .get("id")
            .ok_or_else(|| ObjectError::element_not_found("id"))?;
        let title = properties
            .get("title")
            .ok_or_else(|| ObjectError::element_not_found("title"))?;
        let alternate = properties
            .get("alternate")
            .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

        Ok(Object::Changeset {
            id: id.clone(),
            title: title.clone(),
            alternate: alternate.clone(),
        })
    }

    fn parse_issue(properties: &HashMap<String, String>) -> ObjectResult<Object> {
        let id = properties
            .get("id")
            .ok_or_else(|| ObjectError::element_not_found("id"))?;
        let title = properties
            .get("title")
            .ok_or_else(|| ObjectError::element_not_found("title"))?;
        let summary = properties
            .get("summary")
            .ok_or_else(|| ObjectError::element_not_found("summary"))?;
        let alternate = properties
            .get("alternate")
            .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

        Ok(Object::Issue {
            id: id.clone(),
            title: title.clone(),
            summary: summary.clone(),
            alternate: alternate.clone(),
        })
    }

    fn parse_repository(properties: &HashMap<String, String>) -> ObjectResult<Object> {
        let id = properties
            .get("id")
            .ok_or_else(|| ObjectError::element_not_found("id"))?;
        let title = properties
            .get("title")
            .ok_or_else(|| ObjectError::element_not_found("title"))?;
        let alternate = properties
            .get("alternate")
            .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

        Ok(Object::Repository {
            id: id.clone(),
            title: title.clone(),
            alternate: alternate.clone(),
        })
    }

    fn parse_review(properties: &HashMap<String, String>) -> ObjectResult<Object> {
        let id = properties
            .get("id")
            .ok_or_else(|| ObjectError::element_not_found("id"))?;
        let title = properties
            .get("title")
            .ok_or_else(|| ObjectError::element_not_found("title"))?;
        let summary = properties
            .get("summary")
            .ok_or_else(|| ObjectError::element_not_found("summary"))?;
        let alternate = properties
            .get("alternate")
            .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

        Ok(Object::Review {
            id: id.clone(),
            title: title.clone(),
            summary: summary.clone(),
            alternate: alternate.clone(),
        })
    }

    fn parse_page(properties: &HashMap<String, String>) -> ObjectResult<Object> {
        let id = properties
            .get("id")
            .ok_or_else(|| ObjectError::element_not_found("id"))?;
        let title = properties
            .get("title")
            .ok_or_else(|| ObjectError::element_not_found("title"))?;
        let alternate = properties
            .get("alternate")
            .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

        Ok(Object::Page {
            id: id.clone(),
            title: title.clone(),
            alternate: alternate.clone(),
        })
    }

    fn parse_space(properties: &HashMap<String, String>) -> ObjectResult<Object> {
        let id = properties
            .get("id")
            .ok_or_else(|| ObjectError::element_not_found("id"))?;
        let title = properties
            .get("title")
            .ok_or_else(|| ObjectError::element_not_found("title"))?;
        let alternate = properties
            .get("alternate")
            .ok_or_else(|| ObjectError::element_not_found("alternate"))?;

        Ok(Object::Space {
            id: id.clone(),
            title: title.clone(),
            alternate: alternate.clone(),
        })
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Object::Comment { .. } => write!(f, "comment"),
            Object::File { ref title, .. } => write!(f, "{}", title),
            Object::Person { ref name, .. } => write!(f, "{}", name),
            Object::Changeset { ref title, .. } => write!(f, "{}", title),
            Object::Issue { ref title, .. } => write!(f, "{}", title),
            Object::Repository { ref title, .. } => write!(f, "{}", title),
            Object::Review { ref title, .. } => write!(f, "{}", title),
            Object::Page { ref title, .. } => write!(f, "{}", title),
            Object::Space { ref title, .. } => write!(f, "{}", title),
        }
    }
}
