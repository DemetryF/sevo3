use std::collections::HashMap;

pub struct Scope<V: Clone> {
    data: HashMap<String, V>,
}

impl<V: Clone> Scope<V> {
    pub fn get(&self, id: &str) -> Option<V> {
        self.data.get(id).cloned()
    }

    pub fn add(&mut self, id: String, value: V) {
        self.data.insert(id, value);
    }
}

impl<V: Clone> Default for Scope<V> {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}
