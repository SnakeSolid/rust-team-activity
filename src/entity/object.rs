use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

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
    pub fn comment(id: &str, alternate: &str) -> Object {
        Object::Comment {
            id: id.into(),
            alternate: alternate.into(),
        }
    }

    pub fn file(id: &str, title: &str, alternate: &str) -> Object {
        Object::File {
            id: id.into(),
            title: title.into(),
            alternate: alternate.into(),
        }
    }

    pub fn person(name: &str, email: &str, uri: &str, photo: &str, username: &str) -> Object {
        Object::Person {
            name: name.into(),
            email: email.into(),
            uri: uri.into(),
            photo: photo.into(),
            username: username.into(),
        }
    }

    pub fn changeset(id: &str, title: &str, alternate: &str) -> Object {
        Object::Changeset {
            id: id.into(),
            title: title.into(),
            alternate: alternate.into(),
        }
    }

    pub fn issue(id: &str, title: &str, summary: &str, alternate: &str) -> Object {
        Object::Issue {
            id: id.into(),
            title: title.into(),
            summary: summary.into(),
            alternate: alternate.into(),
        }
    }

    pub fn repository(id: &str, title: &str, alternate: &str) -> Object {
        Object::Repository {
            id: id.into(),
            title: title.into(),
            alternate: alternate.into(),
        }
    }

    pub fn review(id: &str, title: &str, summary: &str, alternate: &str) -> Object {
        Object::Review {
            id: id.into(),
            title: title.into(),
            summary: summary.into(),
            alternate: alternate.into(),
        }
    }

    pub fn page(id: &str, title: &str, alternate: &str) -> Object {
        Object::Page {
            id: id.into(),
            title: title.into(),
            alternate: alternate.into(),
        }
    }

    pub fn space(id: &str, title: &str, alternate: &str) -> Object {
        Object::Space {
            id: id.into(),
            title: title.into(),
            alternate: alternate.into(),
        }
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
