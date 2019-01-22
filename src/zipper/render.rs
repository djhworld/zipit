use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind::InvalidInput;
use std::io::Result;

pub trait ZipOutput {
    fn write(&mut self, left: String, right: String) -> Result<()>;
    fn end(&mut self) -> Result<()>;
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
    fn write(&mut self, left: String, right: String) -> Result<()> {
        if self.buffer.contains_key(&left) {
            let err_msg =
                format!("cannot produce JSON records - attempting to write value to a key that already exists!");
            return Err(Error::new(InvalidInput, err_msg));
        }

        self.buffer.insert(left, right);
        Ok(())
    }

    fn end(&mut self) -> Result<()> {
        let j = serde_json::to_string(&self.buffer);
        println!("{}", j.unwrap());
        self.buffer.clear();
        Ok(())
    }
}

pub struct TabbedOutput {
    buffer: Vec<String>,
}

impl TabbedOutput {
    pub fn new() -> TabbedOutput {
        return TabbedOutput { buffer: Vec::new() };
    }
}

const BLANK: &str = "";

impl ZipOutput for TabbedOutput {
    fn write(&mut self, left: String, right: String) -> Result<()> {
        self.buffer.push(format!("{}\t{}", left, right));
        Ok(())
    }

    fn end(&mut self) -> Result<()> {
        self.buffer.push(BLANK.to_string());
        for i in self.buffer.iter() {
            println!("{}", i);
        }
        self.buffer.clear();
        Ok(())
    }
}
