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

    pub fn save_entry(
        &self,
        id: &str,
        author: &str,
        published: i64,
        data: &str,
    ) -> DatabaseResult<()> {
        let mut statement = self
            .connection
            .prepare("INSERT INTO entry ( id, author, published, data ) VALUES ( ?, ?, ?, ? )")
            .map_err(DatabaseError::prepare_failed)?;
        statement.bind(1, id).map_err(DatabaseError::bind_failed)?;
        statement
            .bind(2, author)
            .map_err(DatabaseError::bind_failed)?;
        statement
            .bind(3, published)
            .map_err(DatabaseError::bind_failed)?;
        statement.bind(4, data).map_err(DatabaseError::bind_failed)?;
        statement.next().map_err(DatabaseError::next_failed)?;

        Ok(())
    }

    pub fn last_published(&self, author: &str) -> DatabaseResult<Option<i64>> {
        let mut statement = self
            .connection
            .prepare("SELECT MAX(published) FROM entry WHERE author = ?")
            .map_err(DatabaseError::prepare_failed)?;
        statement
            .bind(1, author)
            .map_err(DatabaseError::bind_failed)?;
        let mut cursor = statement.cursor();

        if let Some(row) = cursor.next().map_err(DatabaseError::next_failed)? {
            let published = row
                .get(0)
                .ok_or_else(DatabaseError::no_such_column)?
                .as_integer();

            Ok(published)
        } else {
            Ok(None)
        }
    }

    pub fn published_between(
        &self,
        author: &str,
        start_date: i64,
        end_date: i64,
    ) -> DatabaseResult<Vec<String>> {
        let mut statement = self
            .connection
            .prepare("SELECT data FROM entry WHERE author = ? AND published BETWEEN ? AND ?")
            .map_err(DatabaseError::prepare_failed)?;
        statement
            .bind(1, author)
            .map_err(DatabaseError::bind_failed)?;
        statement
            .bind(2, start_date)
            .map_err(DatabaseError::bind_failed)?;
        statement
            .bind(3, end_date)
            .map_err(DatabaseError::bind_failed)?;
        let mut cursor = statement.cursor();
        let mut result = Vec::new();

        while let Some(row) = cursor.next().map_err(DatabaseError::next_failed)? {
            let data = row
                .get(0)
                .ok_or_else(DatabaseError::no_such_column)?
                .as_string()
                .ok_or_else(DatabaseError::no_such_value)?
                .into();

            result.push(data);
        }

        Ok(result)
    }
}

impl Debug for DatabaseInner {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "SQLite database")
    }
}
