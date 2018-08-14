use super::Object;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    author: Object,
    objects: Vec<Object>,
    target: Option<Object>,
    alternate: String,
    application: String,
    content: Option<String>,
    id: String,
    published: String,
    timezone_offset: String,
    title: String,
    updated: String,
    verbs: Vec<String>,
}

impl Entry {
    pub fn new(
        author: Object,
        objects: &[Object],
        target: Option<&Object>,
        alternate: &str,
        application: &str,
        content: Option<&String>,
        id: &str,
        published: &str,
        timezone_offset: &str,
        title: &str,
        updated: &str,
        verbs: &[String],
    ) -> Entry {
        Entry {
            author,
            objects: objects.into(),
            target: target.cloned(),
            alternate: alternate.into(),
            application: application.into(),
            content: content.cloned(),
            id: id.into(),
            published: published.into(),
            timezone_offset: timezone_offset.into(),
            title: title.into(),
            updated: updated.into(),
            verbs: verbs.into(),
        }
    }

    pub fn author(&self) -> &Object {
        &self.author
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub fn target(&self) -> Option<&Object> {
        self.target.as_ref()
    }

    pub fn alternate(&self) -> &str {
        &self.alternate
    }

    pub fn application(&self) -> &str {
        &self.application
    }

    pub fn content(&self) -> Option<&String> {
        self.content.as_ref()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn published(&self) -> &str {
        &self.published
    }

    pub fn timezone_offset(&self) -> &str {
        &self.timezone_offset
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn updated(&self) -> &str {
        &self.updated
    }

    pub fn verbs(&self) -> &[String] {
        &self.verbs
    }
}
