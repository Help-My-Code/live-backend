use serde::{Serialize, Deserialize};
use crate::models::event::Language;

#[derive(Serialize, Debug)]
pub struct ProgramRequest {
  pub stdin: String,
  pub language: Language,
}

#[derive(Deserialize, Debug)]
pub struct ProgramResponse {
  pub stdout: String,
}

