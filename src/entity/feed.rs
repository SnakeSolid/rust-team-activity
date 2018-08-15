use super::Entry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    id: String,
    title: String,
    timezone_offset: String,
    updated: String,
    entries: Vec<Entry>,
}

impl Feed {
    pub fn new(
        id: &str,
        title: &str,
        timezone_offset: &str,
        updated: &str,
        entries: &[Entry],
    ) -> Feed {
        Feed {
            id: id.into(),
            title: title.into(),
            timezone_offset: timezone_offset.into(),
            updated: updated.into(),
            entries: entries.into(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn timezone_offset(&self) -> &str {
        &self.timezone_offset
    }

    pub fn updated(&self) -> &str {
        &self.updated
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
}
