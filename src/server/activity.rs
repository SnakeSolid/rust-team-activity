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
pub struct ActivityResult {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    activity: Option<HashMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl ActivityResult {
    fn with_success(activity: &HashMap<String, Vec<String>>) -> ActivityResult {
        ActivityResult {
            success: true,
            activity: Some(activity.clone()),
            message: None,
        }
    }

    fn with_error(message: &str) -> ActivityResult {
        ActivityResult {
            success: false,
            activity: None,
            message: Some(message.into()),
        }
    }
}

#[derive(Debug)]
pub struct ActivityHandler {
    config: Config,
    database: Database,
}

impl ActivityHandler {
    pub fn new(config: &Config, database: Database) -> ActivityHandler {
        ActivityHandler {
            config: config.clone(),
            database,
        }
    }

    fn published_between(&self, author: &str, activity_date: i64) -> IronResult<Vec<Entry>> {
        let start_date = activity_date;
        let end_date = activity_date + 60 * 60 * 24;
        let result = self
            .database
            .published_between(author, start_date, end_date)
            .unwrap()
            .iter()
            .map(|s| serde_yaml::from_str(s).unwrap())
            .collect();

        Ok(result)
    }
}

macro_rules! try_msg {
    ($ex:expr, $callback:expr) => {
        match $ex {
            Ok(value) => value,
            Err(err) => {
                warn!("{}", err);

                let result = try_err!($callback(err));

                return result;
            }
        }
    };
}

macro_rules! try_err {
    ($ex:expr) => {{
        let response = match serde_json::to_string(&$ex) {
            Ok(body) => Response::with((status::Ok, body)),
            Err(_) => Response::with(status::InternalServerError),
        };

        Ok(response)
    }};
}

impl Handler for ActivityHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();

        try_msg!(req.body.read_to_string(&mut body), |_| {
            ActivityResult::with_error("Incomplete request")
        });

        let activity_date = try_msg!(serde_json::from_str(&body), |_| ActivityResult::with_error(
            "Invalid request, expected UNIX time"
        ));

        let converter = FeedToActivity::new(&self.config);
        let mut result = HashMap::new();

        for author in self.config.members() {
            let entries = self.published_between(author, activity_date)?;
            let activities = converter.convert(&entries);

            for (group, actions) in activities {
                let status = format!("{} - {}", actions.join(", "), group);
                let mut entry = result.entry(author.clone()).or_insert_with(|| Vec::new());

                entry.push(status);
            }
        }

        try_err!(ActivityResult::with_success(&result))
    }
}
