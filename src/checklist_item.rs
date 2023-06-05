use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistItem {
    pub display_name: String,
    pub created_date_time: NaiveDateTime,
    pub is_checked: bool,
    pub id: String,
}
