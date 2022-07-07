use chrono::{DateTime, Utc};

pub struct Room {
  id: String,
  content_id: String,
  program_id: String,
}

pub struct Program {
  id: String,
  stdin: String,
  stdout: String,
  created_at: DateTime<Utc>,
}