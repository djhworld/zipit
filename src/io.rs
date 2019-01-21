use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;

pub struct LineReader {
    buf: BufReader<Box<Read>>,
    _line: String,
}

impl LineReader {
    pub fn new(b: BufReader<Box<Read>>) -> LineReader {
        return LineReader {
            buf: b,
            _line: String::new(),
        };
    }
}

impl Iterator for LineReader {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        self._line.clear();
        return match self.buf.read_line(&mut self._line) {
            Ok(0) => None,
            Ok(_) => Some(Ok(self._line.trim_end().to_owned())),
            Err(err) => Some(Err(err)),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_lines() {
        let data = "foo\nbar\nbaz".as_bytes();
        let mut line_reader = LineReader::new(BufReader::new(Box::new(data) as Box<Read>));
        assert_eq!(line_reader.next().unwrap().unwrap(), "foo");
        assert_eq!(line_reader.next().unwrap().unwrap(), "bar");
        assert_eq!(line_reader.next().unwrap().unwrap(), "baz");
        assert_eq!(line_reader.next().is_none(), true);
    }

    #[test]
    fn should_return_empty_on_empty_input() {
        let data = "".as_bytes();
        let mut line_reader = LineReader::new(BufReader::new(Box::new(data) as Box<Read>));
        assert_eq!(line_reader.next().is_none(), true);
    }
}
