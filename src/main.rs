#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate env_logger;
extern crate iron;
extern crate rand;
extern crate reqwest;
extern crate serde_yaml;
extern crate sqlite;
extern crate time;
extern crate xml;

mod config;
mod context;
mod database;
mod entity;
mod stream;

use config::Config;
use stream::ActivityStreamsClient;
use stream::FeedToActivity;
use time::strptime;
use time::Duration;

fn main() {
    env_logger::init();

    let config = match Config::from_args() {
        Ok(config) => config,
        Err(err) => panic!("Failed to create configuration: {}", err),
    };

    let client = ActivityStreamsClient::new(&config);
    let converter = FeedToActivity::new(&config);

    let tm = strptime("2018-08-15", "%Y-%m-%d").unwrap();
    let start_time = tm.to_timespec();
    let end_time = start_time + Duration::days(1);
    let start_timestamp = start_time.sec * 1000;
    let end_timestamp = end_time.sec * 1000;

    for member in config.members() {
        match client.query_between(member, start_timestamp, end_timestamp) {
            Ok(text) => {
                let bytes = text.as_bytes();
                let feed = entity::read(bytes).unwrap();
                let entries = feed.entries();

                println!("{}", member);

                for (group, messages) in converter.convert(entries) {
                    println!("* {} - {}", messages.join(", "), group);
                }

                println!("----------------");
            }
            Err(err) => {
                warn!("Activity stream error: {}", err);
            }
        }
    }
}
