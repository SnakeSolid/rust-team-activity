use serde_yaml::from_reader;
use std::collections::HashMap;
use std::env::args;
use std::fs::File;

mod error;

use self::error::ConfigError;
use self::error::ConfigResult;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    streams: StreamsConfig,
    database: DatabaseConfig,
    #[serde(default = "default_server")]
    server: ServerConfig,
    members: Vec<String>,
    activity: ActivitiesConfig,
    #[serde(default = "default_start_worker")]
    start_worker: bool,
    #[serde(default = "default_pull_interval")]
    pull_interval: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamsConfig {
    url: String,
    #[serde(default = "default_max_results")]
    max_results: usize,
    username: String,
    password: Option<String>,
    #[serde(default = "default_root_certificates")]
    root_certificates: Vec<String>,
    #[serde(default = "default_hostname_verification")]
    hostname_verification: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_address")]
    address: String,
    #[serde(default = "default_port")]
    port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActivitiesConfig {
    ignore: Vec<IgnoreConfig>,
    activities: Vec<ActivityConfig>,
    messages: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IgnoreConfig {
    application: Option<String>,
    verbs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ActivityConfig {
    application: Option<String>,
    key: String,
    group: MessageGroup,
    verbs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageConfig {
    key: String,
    messages: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum MessageGroup {
    TargetIssue,
    TargetReview,
    TargetPage,
    ObjectIssue,
    ObjectReview,
    ObjectPage,
    Content,
}

impl Config {
    /// Returns configuration created from first application argument or from file `config.yaml`.
    pub fn from_args() -> ConfigResult<Config> {
        let path = args()
            .skip(1)
            .take(1)
            .next()
            .unwrap_or_else(|| "config.yaml".into());
        let file = File::open(path).map_err(ConfigError::io_error)?;
        let config: Config = from_reader(file).map_err(ConfigError::deserialization_failed)?;

        Ok(config)
    }

    pub fn streams(&self) -> &StreamsConfig {
        &self.streams
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn activity(&self) -> &ActivitiesConfig {
        &self.activity
    }

    pub fn members(&self) -> &[String] {
        &self.members
    }

    pub fn start_worker(&self) -> bool {
        self.start_worker
    }

    pub fn pull_interval(&self) -> u64 {
        self.pull_interval
    }
}

impl StreamsConfig {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn max_results(&self) -> usize {
        self.max_results
    }

    pub fn username(&self) -> &str {
        &self.username.as_ref()
    }

    pub fn password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    pub fn root_certificates(&self) -> &[String] {
        &self.root_certificates
    }

    pub fn hostname_verification(&self) -> bool {
        self.hostname_verification
    }
}

impl DatabaseConfig {
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl ServerConfig {
    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

impl ActivitiesConfig {
    pub fn ignore(&self) -> &[IgnoreConfig] {
        &self.ignore
    }

    pub fn activities(&self) -> &[ActivityConfig] {
        &self.activities
    }

    pub fn messages(&self) -> &HashMap<String, Vec<String>> {
        &self.messages
    }
}

impl IgnoreConfig {
    pub fn application(&self) -> Option<&String> {
        self.application.as_ref()
    }

    pub fn verbs(&self) -> &[String] {
        &self.verbs
    }
}

impl ActivityConfig {
    pub fn application(&self) -> Option<&String> {
        self.application.as_ref()
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn group(&self) -> MessageGroup {
        self.group
    }

    pub fn verbs(&self) -> &[String] {
        &self.verbs
    }
}

#[inline]
fn default_pull_interval() -> u64 {
    3600
}

#[inline]
fn default_max_results() -> usize {
    25
}

fn default_server() -> ServerConfig {
    ServerConfig {
        address: default_address(),
        port: default_port(),
    }
}

#[inline]
fn default_address() -> String {
    "localhost".into()
}

#[inline]
fn default_port() -> u16 {
    8080
}

#[inline]
fn default_root_certificates() -> Vec<String> {
    Vec::with_capacity(0)
}

#[inline]
fn default_hostname_verification() -> bool {
    true
}

#[inline]
fn default_start_worker() -> bool {
    true
}
