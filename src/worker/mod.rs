use serde_yaml;
use std::thread;
use std::thread::Builder;
use std::thread::JoinHandle;
use std::time::Duration as StdDuration;
use std::time::Instant;
use time::strptime;
use time::Duration;
use time::Timespec;

use config::Config;
use database::Database;
use entity;
use stream::ActivityStreamsClient;

mod error;

use self::error::WorkerError;
use self::error::WorkerResult;

#[derive(Debug)]
pub struct Worker {
    config: Config,
    pull_interval: u64,
    members: Vec<String>,
    database: Database,
}

impl Worker {
    pub fn new(config: &Config, database: Database) -> Worker {
        let pull_interval = config.pull_interval();
        let members = config.members().into();

        Worker {
            config: config.clone(),
            pull_interval,
            members,
            database,
        }
    }

    pub fn start(self) -> JoinHandle<()> {
        println!("Starting background worker...");

        Builder::new()
            .name("Stream worker".into())
            .spawn(move || self.run())
            .expect("Failed to start worker thread")
    }

    fn run(self) {
        let client = ActivityStreamsClient::new(&self.config);

        info!("Worker started");

        let mut start_time = Instant::now();
        let mut end_time;
        let interval = StdDuration::from_secs(self.pull_interval);

        loop {
            end_time = start_time;

            if let Err(err) = self.update_activity(&client) {
                warn!("Failed to update last activity: {}", err);
            }

            start_time = Instant::now();

            let elapsed = start_time.duration_since(end_time);

            info!("Worker round time {}", elapsed.as_secs());

            if elapsed < interval {
                let sleep_interval = interval - elapsed;

                debug!("Worker sleeping for {}", sleep_interval.as_secs());

                thread::sleep(sleep_interval);
            }
        }
    }

    fn update_activity(&self, client: &ActivityStreamsClient) -> WorkerResult<()> {
        let interval = Duration::seconds(self.pull_interval as i64);

        for member in &self.members {
            info!("Processing {}", member);

            let last_published = self
                .database
                .last_published(member)
                .map_err(WorkerError::database_error)?;
            let result = if let Some(published) = last_published {
                let published_time = Timespec::new(published, 0);
                let start_time = published_time - interval;

                debug!("Query activity for {} since {}", member, start_time.sec);

                client
                    .query_after(member, start_time.sec * 1000)
                    .map_err(WorkerError::stream_error)?
            } else {
                debug!("Query all activity for {}", member);

                client.query(member).map_err(WorkerError::stream_error)?
            };
            let bytes = result.as_bytes();
            let feed = entity::read(bytes).map_err(WorkerError::entity_read_error)?;
            let entries = feed.entries();

            for entry in entries {
                let id = entry.id();

                debug!("Processing entry {}", id);

                if !self
                    .database
                    .has_entry(id)
                    .map_err(WorkerError::database_error)?
                {
                    let published = strptime(entry.published(), "%Y-%m-%dT%H:%M:%S")
                        .map_err(WorkerError::date_parse_error)?
                        .to_timespec();
                    let data =
                        serde_yaml::to_string(entry).map_err(WorkerError::serialization_error)?;

                    self.database
                        .save_entry(id, member, published.sec, &data)
                        .map_err(WorkerError::database_error)?;

                    debug!("Entry saved: {}", id);
                } else {
                    debug!("Entry already processed: {}", id);
                }
            }
        }

        Ok(())
    }
}
