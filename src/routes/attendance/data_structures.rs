use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Attendance {
    pub(super) date: NaiveDate,
    pub(super) time: NaiveTime,
}
