use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Program {
    pub name: String,
    pub command: String,
    pub environment: Vec<String>,
}

impl Program {
    pub fn envs(&self) -> HashMap<String, String> {
        self.environment
            .iter()
            .map(|s| s.split_at(s.find("=").unwrap()))
            .map(|(key, val)| (key.to_string(), val[1..].to_string()))
            .collect()
    }
}
