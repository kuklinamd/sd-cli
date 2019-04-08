use std::collections::HashMap;

/// Local environement.
///
/// Used only for string interpolation. We dont' export
/// these variables, so they're not in the environement
/// of subsequently executed commands.
pub struct Env(HashMap<String, String>);

impl Env {
    pub fn new() -> Env {
        Env(HashMap::new())
    }

    /// Add a new variable with given `name` and `value`.
    pub fn add(&mut self, name: String, value: String) {
        self.0.insert(name, value);
    }

    /// Get variable with given `name`.
    pub fn get(&self, name: &String) -> Option<String> {
        self.0.get(name).cloned()
    }
}
