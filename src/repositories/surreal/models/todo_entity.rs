use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoEntity {
  pub id: Option<Thing>,
  pub title: String,
  pub content: String,
  pub completed: Option<bool>,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
}
