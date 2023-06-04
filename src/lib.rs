#[derive(Debug)]
pub struct KvStore {}

impl KvStore {
    // declare a new instance of the KvStore
    pub fn new() -> KvStore {
        unimplemented!()
    }

    pub fn set<'a>(&mut self, key: String, val: String) {
        unimplemented!()
    }

    pub fn remove(&mut self, key: String) {
        unimplemented!()
    }

    pub fn get(&self, key: String) -> Option<String> {
        unimplemented!()
    }
}
