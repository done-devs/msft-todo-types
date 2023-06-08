use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeTimeZone {
    pub date_time: String,
    pub time_zone: String,
}

impl From<DateTimeTimeZone> for DateTime<Utc> {
    fn from(date: DateTimeTimeZone) -> Self {
        DateTime::<Utc>::from_str(&date.date_time).unwrap()
    }
}

impl From<DateTime<Utc>> for DateTimeTimeZone {
    fn from(date: DateTime<Utc>) -> Self {
        Self {
            date_time: date.to_string(),
            time_zone: date.timezone().to_string(),
        }
    }
}
