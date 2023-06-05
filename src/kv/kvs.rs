use std::collections::HashMap;

/// The `KvStore` stores strings key value pairs in memory
///
/// The Key/value pairs are stored in a `HashMap`
///
/// Example
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Debug)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// declare a new instance of the `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// Sets the value of the provided string.
    ///
    /// if the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, val: String) {
        self.store.insert(key, val);
    }

    /// Removes the given key and its value
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }

    /// Gets the value based on the given string key.
    ///
    /// Returns `None` if the given key doesn't exist.
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }
}
