use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    body::Body, checklist_item::ChecklistItem, importance::Importance, recurrence::Recurrence,
    status::Status,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ToDoTask {
    pub id: String,
    pub body: Body,
    pub categories: Vec<String>,
    pub completed_date_time: Option<DateTime<Utc>>,
    pub due_date_time: Option<DateTime<Utc>>,
    pub importance: Importance,
    pub is_reminder_on: bool,
    pub recurrence: Recurrence,
    pub title: String,
    pub status: Status,
    pub has_attachments: bool,
    pub checklist_items: Vec<ChecklistItem>,
    pub body_last_modified_date_time: Option<DateTime<Utc>>,
    pub created_date_time: DateTime<Utc>,
    pub last_modified_date_time: DateTime<Utc>,
    pub reminder_date_time: Option<DateTime<Utc>>,
}
