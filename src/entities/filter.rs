use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Filter {
    pub id: String,
    pub phrase: String,
    pub context: Vec<FilterContext>,
    pub expires_at: Option<DateTime<Utc>>,
    pub irreversible: bool,
    pub whole_word: bool,
}

#[derive(Debug, Serialize, Clone)]
pub enum FilterContext {
    Home,
    Notifications,
    Public,
    Thread,
}
