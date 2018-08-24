#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate env_logger;
extern crate iron;
extern crate mount;
extern crate rand;
extern crate reqwest;
extern crate router;
extern crate serde_json;
extern crate serde_yaml;
extern crate sqlite;
extern crate staticfile;
extern crate time;
extern crate xml;

mod config;
mod database;
mod entity;
mod server;
mod stream;
mod worker;

use config::Config;
use database::Database;
use worker::Worker;

fn main() {
    env_logger::init();

    let config = match Config::from_args() {
        Ok(config) => config,
        Err(err) => panic!("Failed to create configuration: {}", err),
    };

    let database = match Database::new(&config) {
        Ok(database) => database,
        Err(err) => panic!("Failed to initialize database: {}", err),
    };

    let join_worker = if config.start_worker() {
        let database = database.clone();
        let worker = Worker::new(&config, database);

        Some(worker.start())
    } else {
        None
    };

    server::start(&config, database);

    drop(join_worker);
}
