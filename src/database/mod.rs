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

#[derive(Debug)]
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
}
