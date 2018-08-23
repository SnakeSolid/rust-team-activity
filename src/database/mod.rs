use sqlite;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

mod error;
mod inner;

pub use self::error::DatabaseError;
pub use self::error::DatabaseResult;

use self::inner::DatabaseInner;
use Config;

#[derive(Debug, Clone)]
pub struct Database {
    inner: Arc<Mutex<DatabaseInner>>,
}

impl Database {
    pub fn new(config: &Config) -> DatabaseResult<Database> {
        let path = config.database().path();
        let path: &Path = path.as_ref();
        let initialize = !path.exists();
        let connection = sqlite::open(path).map_err(DatabaseError::connection_error)?;
        let mut inner = DatabaseInner::new(connection);

        if initialize {
            inner.create_database()?;
        }

        Ok(Database {
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    pub fn has_entry(&self, id: &str) -> DatabaseResult<bool> {
        self.inner
            .lock()
            .map_err(DatabaseError::mutex_lock_error)?
            .has_entry(id)
    }

    pub fn save_entry(
        &self,
        id: &str,
        author: &str,
        published: i64,
        data: &str,
    ) -> DatabaseResult<()> {
        self.inner
            .lock()
            .map_err(DatabaseError::mutex_lock_error)?
            .save_entry(id, author, published, data)
    }

    pub fn last_published(&self, author: &str) -> DatabaseResult<Option<i64>> {
        self.inner
            .lock()
            .map_err(DatabaseError::mutex_lock_error)?
            .last_published(author)
    }

    pub fn published_between(
        &self,
        author: &str,
        start_date: i64,
        end_date: i64,
    ) -> DatabaseResult<Vec<String>> {
        self.inner
            .lock()
            .map_err(DatabaseError::mutex_lock_error)?
            .published_between(author, start_date, end_date)
    }
}
