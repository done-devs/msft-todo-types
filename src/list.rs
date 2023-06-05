use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ToDoTaskList {
    pub display_name: String,
    pub id: String,
    pub is_owner: String,
    pub is_shared: String,
    pub wellknown_list_name: String,
}
