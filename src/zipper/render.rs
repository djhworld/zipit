use std::collections::HashMap;

pub trait ZipOutput {
    fn write(&mut self, left: String, right: String);
    fn end(&mut self);
}

pub struct JsonOuput {
    buffer: HashMap<String, String>,
}

impl JsonOuput {
    pub fn new() -> JsonOuput {
        return JsonOuput {
            buffer: HashMap::new(),
        };
    }
}

impl ZipOutput for JsonOuput {
    fn write(&mut self, left: String, right: String) {
        if self.buffer.contains_key(&left) {
            panic!("Attempting to write value to a key that already exists on the left hand side");
        }

        self.buffer.insert(left, right);
    }

    fn end(&mut self) {
        let j = serde_json::to_string(&self.buffer);
        println!("{}", j.unwrap());
        self.buffer.clear();
    }
}

pub struct TabOutput {}

impl TabOutput {
    pub fn new() -> TabOutput {
        return TabOutput {};
    }
}

impl ZipOutput for TabOutput {
    fn write(&mut self, left: String, right: String) {
        println!("{}\t{}", left, right);
    }

    fn end(&mut self) {
        println!("");
    }
}
