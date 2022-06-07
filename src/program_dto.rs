use serde::Serialize;

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
