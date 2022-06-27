use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Delta {
  action: String,
  start: Point,
  end: Point,
  lines: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
  row: usize,
  column: usize,
}