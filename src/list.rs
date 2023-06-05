use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ToDoTaskList {
    pub display_name: String,
    pub id: String,
    pub is_owner: bool,
    pub is_shared: bool,
    pub wellknown_list_name: String,
}
