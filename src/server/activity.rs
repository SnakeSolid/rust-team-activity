use iron::middleware::Handler;
use iron::status;
use iron::IronResult;
use iron::Request;
use iron::Response;
use serde_json;
use serde_yaml;
use std::collections::HashMap;
use std::io::Read;

use config::Config;
use database::Database;
use entity::Entry;
use stream::FeedToActivity;

#[derive(Debug, Clone, Serialize)]
pub struct ActivytyResult {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    activity: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl ActivytyResult {
    fn with_success(activity: &HashMap<String, Vec<String>>) -> ActivytyResult {
        ActivytyResult {
            success: true,
            activity: Some(activity.clone()),
            message: None,
        }
    }

    fn with_error(message: &str) -> ActivytyResult {
        ActivytyResult {
            success: false,
            activity: None,
            message: Some(message.into()),
        }
    }
}

#[derive(Debug)]
pub struct ActivytyHandler {
    config: Config,
    database: Database,
}

impl ActivytyHandler {
    pub fn new(config: &Config, database: Database) -> ActivytyHandler {
        ActivytyHandler {
            config: config.clone(),
            database,
        }
    }

    fn published_between(&self, author: &str, activity_date: i64) -> Vec<Entry> {
        let start_date = activity_date;
        let end_date = activity_date + 60 * 60 * 24;

        self.database
            .published_between(author, start_date, end_date)
            .unwrap()
            .iter()
            .map(|s| serde_yaml::from_str(s).unwrap())
            .collect()
    }
}

impl Handler for ActivytyHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();

        req.body.read_to_string(&mut body).unwrap();

        let activity_date = serde_json::from_str(&body).unwrap();
        let converter = FeedToActivity::new(&self.config);
        let mut result = HashMap::new();

        for author in self.config.members() {
            let entries = self.published_between(author, activity_date);
            let activities = converter.convert(&entries);

            for (group, actions) in activities {
                let status = format!("{} - {}", actions.join(", "), group);
                let mut entry = result.entry(author.clone()).or_insert_with(|| Vec::new());

                entry.push(status);
            }
        }

        Ok(Response::with((
            status::Ok,
            serde_json::to_string(&ActivytyResult::with_success(&result)).unwrap(),
        )))
    }
}
