use std::collections::HashMap;

/// Local environement.
pub struct Env(HashMap<String, String>);

impl Env {
    pub fn new() -> Env {
        Env(HashMap::new())
    }

    pub fn add(&mut self, name: String, value: String) {
        self.0.insert(name, value);
    }

    pub fn get(&self, name: &String) -> Option<String> {
        self.0.get(name).cloned()
    }
}
