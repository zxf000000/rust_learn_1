use chrono::{serde::ts_seconds, DateTime, Utc, Local};
use serde;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let create_at: DateTime<Utc> = Utc::now();
        Task {text, created_at}
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let create_at = self.created_at.with_time_zone(&Local).format("&F &H:&M");
        write!(f, "{:<50} [{}]", self.text, create_at)
    }
}