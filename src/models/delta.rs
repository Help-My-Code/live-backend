use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Delta {
    start: Point,
    end: Point,
    action: String,
    lines: Vec<String>,
    timestamp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    row: usize,
    column: usize,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_delta_serialization() {
        let delta = Delta {
            action: "insert".to_string(),
            start: Point { row: 5, column: 17 },
            end: Point { row: 5, column: 19 },
            lines: vec!["Hello, world!".to_string()],
            timestamp: 0.0,
        };
        let serialized = serde_json::to_string::<Delta>(&delta).unwrap();
        assert_eq!(serialized, "{\"action\":\"insert\",\"start\":{\"row\":5,\"column\":17},\"end\":{\"row\":5,\"column\":19},\"lines\":[\"Hello, world!\"]}");
    }

    #[test]
    fn test_delta_deserialization() {
        let serialized = "{\"action\":\"insert\",\"start\":{\"row\":5,\"column\":17},\"end\":{\"row\":5,\"column\":19},\"lines\":[\"Hello, world!\"], \"timestamp\":0.0}";
        let delta: Delta = serde_json::from_str(serialized).unwrap();
        assert_eq!(delta.action, "insert");
        assert_eq!(delta.start.row, 5);
        assert_eq!(delta.start.column, 17);
        assert_eq!(delta.end.row, 5);
        assert_eq!(delta.end.column, 19);
        assert_eq!(delta.lines[0], "Hello, world!");
        assert_eq!(delta.timestamp, 0.0);
    }
}
