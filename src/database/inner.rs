use sqlite::Connection;
use sqlite::State;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use super::DatabaseError;
use super::DatabaseResult;

pub struct DatabaseInner {
    connection: Connection,
}

impl DatabaseInner {
    pub fn new(connection: Connection) -> DatabaseInner {
        DatabaseInner { connection }
    }

    pub fn create_database(&mut self) -> DatabaseResult<()> {
        self.connection
            .execute(include_str!("init_database.sql"))
            .map_err(DatabaseError::execution_error)
    }

    pub fn has_entry(&self, id: &str) -> DatabaseResult<bool> {
        let mut statement = self
            .connection
            .prepare("SELECT id FROM entry WHERE id = ?")
            .map_err(DatabaseError::prepare_failed)?;
        statement.bind(1, id).map_err(DatabaseError::bind_failed)?;

        if let State::Row = statement.next().map_err(DatabaseError::next_failed)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Debug for DatabaseInner {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "SQLite database")
    }
}
