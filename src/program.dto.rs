pub enum Language {
  DART,
  PYTHON,
  C,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramRequest {
  pub stdin: String,
  pub language: Language,
}