use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
}

// ============ Tasks ============

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
    pub archived: bool,
    pub sort_order: i32,
    pub recurrence: Option<RecurrenceRule>,
    pub subtasks: Option<Vec<Subtask>>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<RecurrenceRule>,
}

#[derive(Debug, Serialize)]
pub struct UpdateTask {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<RecurrenceRule>,
}

#[derive(Debug, Serialize)]
pub struct AddSubtask {
    pub title: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderTasks {
    pub task_id: String,
    pub new_index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportTask {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<ImportSubtask>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportSubtask {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportTasks {
    pub tasks: Vec<ImportTask>,
}

// ============ Events ============

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
    pub description: String,
    pub notes: String,
    pub tags: Vec<String>,
    pub recurrence: Option<RecurrenceRule>,
    pub reminder_minutes: Option<i32>,
    pub end_date: Option<String>,
    pub excluded_dates: Option<Vec<String>>,
    pub detached_from_id: Option<String>,
    pub subtasks: Option<Vec<EventSubtask>>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct EventSubtask {
    pub id: String,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventSubtask {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportEvent {
    pub title: String,
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<RecurrenceRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_dates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<ImportEventSubtask>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportEventSubtask {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportEvents {
    pub events: Vec<ImportEvent>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEvent {
    pub title: String,
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<RecurrenceRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_dates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_from_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<CreateEventSubtask>>,
}

#[derive(Debug, Serialize)]
pub struct UpdateEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<RecurrenceRule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_dates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_from_id: Option<String>,
}

// ============ Habits ============

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateHabit {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateHabit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportHabits {
    pub habits: Vec<CreateHabit>,
}

// ============ Recurrence ============

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurrenceRule {
    #[serde(rename = "type")]
    pub recurrence_type: String, // daily, weekly, monthly, quarterly, semiannual, yearly
    pub interval: u32,
    pub end_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_of_week: Option<Vec<u8>>, // 0-6 (Dim-Sam), pour weekly
}

// ============ Images ============

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Image {
    pub id: String,
    pub original_name: String,
    pub mime_type: String,
    pub size: u64,
    pub task_id: Option<String>,
    pub event_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct GalleryImage {
    pub id: String,
    pub original_name: String,
    pub mime_type: String,
    pub size: u64,
    pub task_id: Option<String>,
    pub event_id: Option<String>,
    pub entity_name: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ImageQuota {
    pub used: u64,
    pub limit: u64,
    pub count: u64,
}

// ============ API Tokens ============

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct ApiToken {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub expires_at: Option<String>,
    pub last_used_at: Option<String>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>, // Only returned on creation
}

#[derive(Debug, Serialize)]
pub struct CreateToken {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_days: Option<u32>,
}
