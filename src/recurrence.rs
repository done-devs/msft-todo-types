use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Recurrence {
    pub pattern: RecurrencePattern,
    pub range: RecurrenceRange,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct RecurrenceRange {
    pub end_date: NaiveDate,
    pub number_of_occurrences: i32,
    pub recurrence_time_zone: String,
    pub start_date: NaiveDate,
    #[serde(rename = "type")]
    pub recurrence_range_type: RecurrenceRangeType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum RecurrenceRangeType {
    #[default]
    EndDate,
    NoEnd,
    Numbered,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct RecurrencePattern {
    pub day_of_month: i32,
    pub days_of_week: Vec<DayOfWeek>,
    pub first_day_of_week: DayOfWeek,
    pub index: Option<WeekIndex>,
    pub interval: i32,
    pub month: i32,
    #[serde(rename = "type")]
    pub recurrence_pattern_type: RecurrencePatternType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum DayOfWeek {
    #[default]
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum WeekIndex {
    #[default]
    First,
    Second,
    Third,
    Fourth,
    Last,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum RecurrencePatternType {
    #[default]
    Daily,
    Weekly,
    AbsoluteMonthly,
    RelativeMonthly,
    AbsoluteYearly,
    RelativeYearly,
}
