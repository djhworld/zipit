use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use structopt::StructOpt;

use zipit::LineReader;

/// Takes a line from each input file and prints them to stdout, until either file reaches EOF
#[derive(StructOpt)]
#[structopt(name = "zipit")]
struct Opt {
    /// Delimiter to use for output (<TAB>)
    #[structopt(short = "d", long = "delimiter", default_value = "\t")]
    delimiter: String,

    #[structopt(name = "FILE1")]
    file1: String,

    #[structopt(name = "FILE2")]
    file2: String,
}

fn open_files_as_line_readers(file1: &str, file2: &str) -> Result<(LineReader, LineReader)> {
    let buffer1 = BufReader::new(Box::new(File::open(file1)?) as Box<Read>);
    let buffer2 = BufReader::new(Box::new(File::open(file2)?) as Box<Read>);

    let lr1 = LineReader::new(buffer1);
    let lr2 = LineReader::new(buffer2);

    return Ok((lr1, lr2));
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let readers = open_files_as_line_readers(opt.file1.as_str(), opt.file2.as_str())?;

    for items in readers.0.zip(readers.1) {
        let left = items.0?;
        let right = items.1?;
        println!("{}{}{}", left, opt.delimiter, right);
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_zip_buffers() {
        let data1 = "foo\nbar".as_bytes();
        let data2 = "1\n2".as_bytes();

        let expected: Vec<(String, String)> = vec![
            ("foo".to_owned(), "1".to_owned()),
            ("bar".to_owned(), "2".to_owned()),
        ];

        let items = _create_zip_results_for(data1, data2);

        assert_eq!(items.len(), expected.len());

        for (index, item) in items.iter().enumerate() {
            let actual_left = item.0.as_ref().unwrap();
            let actual_right = item.1.as_ref().unwrap();

            assert_eq!(actual_left, &expected[index].0);
            assert_eq!(actual_right, &expected[index].1);
        }
    }

    #[test]
    fn should_return_nothing_on_empty_buffers() {
        let data1 = "".as_bytes();
        let data2 = "".as_bytes();

        let items = _create_zip_results_for(data1, data2);

        assert!(items.len() == 0);
    }

    #[test]
    fn should_not_error_if_buffer2_is_larger_than_buffer1() {
        let data1 = "foo\nbar".as_bytes();
        let data2 = "1\n2\n3\n4".as_bytes();

        let expected: Vec<(String, String)> = vec![
            ("foo".to_owned(), "1".to_owned()),
            ("bar".to_owned(), "2".to_owned()),
        ];

        let items = _create_zip_results_for(data1, data2);

        assert_eq!(items.len(), expected.len());

        for (index, item) in items.iter().enumerate() {
            let actual_left = item.0.as_ref().unwrap();
            let actual_right = item.1.as_ref().unwrap();

            assert_eq!(actual_left, &expected[index].0);
            assert_eq!(actual_right, &expected[index].1);
        }
    }

    #[test]
    fn should_not_error_if_buffer1_is_larger_than_buffer2() {
        let data1 = "foo\nbar\nbaz\nbee".as_bytes();
        let data2 = "1\n2".as_bytes();

        let expected: Vec<(String, String)> = vec![
            ("foo".to_owned(), "1".to_owned()),
            ("bar".to_owned(), "2".to_owned()),
        ];

        let items = _create_zip_results_for(data1, data2);

        assert_eq!(items.len(), expected.len());

        for (index, item) in items.iter().enumerate() {
            let actual_left = item.0.as_ref().unwrap();
            let actual_right = item.1.as_ref().unwrap();

            assert_eq!(actual_left, &expected[index].0);
            assert_eq!(actual_right, &expected[index].1);
        }
    }

    fn _create_zip_results_for(
        data1: &'static [u8],
        data2: &'static [u8],
    ) -> Vec<(Result<String>, Result<String>)> {
        let lr1 = LineReader::new(BufReader::new(Box::new(data1) as Box<Read>));
        let lr2 = LineReader::new(BufReader::new(Box::new(data2) as Box<Read>));

        let result = lr1.zip(lr2);

        let mut items = Vec::new();

        for item in result {
            items.push(item);
        }

        return items;
    }
}
