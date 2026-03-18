use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub category: String,
    pub status: String,
    pub priority: String,
    pub due_date: Option<String>,
    pub notes: String,
    pub tags: Vec<String>,
    pub subtasks: Option<Vec<Subtask>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Subtask {
    pub id: String,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTask {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct UpdateTask {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "dueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AddSubtask {
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub category: String,
    pub date: String,
    pub start_time: String,
    pub end_time: String,
    pub all_day: bool,
    pub description: String,
    pub notes: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEvent {
    pub title: String,
    pub date: String,
    pub start_time: String,
    pub end_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_day: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Habit {
    pub id: String,
    pub title: String,
    pub category: String,
    pub frequency: String,
    pub completed_dates: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateHabit {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}
