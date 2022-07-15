use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub enum Language {
  DART,
  PYTHON,
  C,
}

#[derive(Serialize, Debug)]
pub struct ProgramRequest {
  pub stdin: String,
  pub language: Language,
}

#[derive(Deserialize, Debug)]
pub struct ProgramResponse {
  pub stdout: String,
}

