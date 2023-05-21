use std::collections::HashMap;

pub type SstpHeader = HashMap<String, String>;

pub struct SstpRequest {
    headers: SstpHeader,
}

impl SstpRequest {
    pub fn new() -> Self {
        Self {
            headers: SstpHeader::new(),
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers.get(key).cloned()
    }

    pub fn method(&self) -> Option<String> {
        self.get_header("method")
    }

    pub fn id(&self) -> Option<String> {
        self.get_header("ID")
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();

        for (key, value) in &self.headers {
            ret.push_str(&format!("{}: {}\r\n", key, value));
        }

        ret
    }
}
