use reqwest::Certificate;
use reqwest::Client;
use std::fs::File;
use std::io::Read;

use super::error::ActivityStreamsError;
use super::error::ActivityStreamsResult;

use config::Config;

#[derive(Debug)]
pub struct ActivityStreamsClient {
    client: Client,
    url: String,
    max_results: usize,
    username: String,
    password: Option<String>,
}

impl ActivityStreamsClient {
    pub fn new(config: &Config) -> ActivityStreamsClient {
        let streams = config.streams();
        let mut builder = Client::builder();

        for cetrificate_path in streams.root_certificates() {
            debug!("Loading certificate {}", cetrificate_path);

            let mut buffer = Vec::new();

            File::open(cetrificate_path)
                .unwrap()
                .read_to_end(&mut buffer)
                .unwrap();

            let certificate = Certificate::from_pem(&buffer).unwrap();

            builder.add_root_certificate(certificate);
        }

        if !streams.hostname_verification() {
            debug!("Disable host name verification");

            builder.danger_disable_hostname_verification();
        }

        ActivityStreamsClient {
            client: builder.build().unwrap(),
            url: streams.url().into(),
            max_results: streams.max_results(),
            username: streams.username().into(),
            password: streams.password().cloned(),
        }
    }

    pub fn query(&self, user_name: &str) -> ActivityStreamsResult<String> {
        let max_results = format!("{}", &self.max_results);
        let streams_user = format!("user IS {}", user_name,);
        let mut responce = self
            .client
            .get(&self.url)
            .basic_auth(self.username.clone(), self.password.clone())
            .query(&[("maxResults", &max_results), ("streams", &streams_user)])
            .send()
            .map_err(ActivityStreamsError::request_error)?;
        let status = responce.status();

        if status.is_success() {
            responce.text().map_err(ActivityStreamsError::request_error)
        } else {
            Err(ActivityStreamsError::status_not_success(status))
        }
    }

    pub fn query_between(
        &self,
        user_name: &str,
        start_time: i64,
        end_time: i64,
    ) -> ActivityStreamsResult<String> {
        let max_results = format!("{}", &self.max_results);
        let streams_user = format!("user IS {}", user_name,);
        let streams_update = format!("update-date BETWEEN {} {}", start_time, end_time);
        let mut responce = self
            .client
            .get(&self.url)
            .basic_auth(self.username.clone(), self.password.clone())
            .query(&[
                ("maxResults", &max_results),
                ("streams", &streams_user),
                ("streams", &streams_update),
            ])
            .send()
            .map_err(ActivityStreamsError::request_error)?;
        let status = responce.status();

        if status.is_success() {
            responce.text().map_err(ActivityStreamsError::request_error)
        } else {
            Err(ActivityStreamsError::status_not_success(status))
        }
    }

    pub fn query_after(&self, user_name: &str, start_time: i64) -> ActivityStreamsResult<String> {
        let max_results = format!("{}", &self.max_results);
        let streams_user = format!("user IS {}", user_name,);
        let streams_update = format!("update-date AFTER {}", start_time);
        let mut responce = self
            .client
            .get(&self.url)
            .basic_auth(self.username.clone(), self.password.clone())
            .query(&[
                ("maxResults", &max_results),
                ("streams", &streams_user),
                ("streams", &streams_update),
            ])
            .send()
            .map_err(ActivityStreamsError::request_error)?;
        let status = responce.status();

        if status.is_success() {
            responce.text().map_err(ActivityStreamsError::request_error)
        } else {
            Err(ActivityStreamsError::status_not_success(status))
        }
    }
}
