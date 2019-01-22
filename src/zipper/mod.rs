pub mod render;

use render::ZipOutput;
use std::io::Result;

pub trait Zipper {
    fn zip(
        &self,
        left_iter: Box<Iterator<Item = Result<String>>>,
        right_iter: Box<Iterator<Item = Result<String>>>,
        output: Box<&mut ZipOutput>,
    ) -> Result<()>;
}

pub struct DefaultZipper {}

impl Zipper for DefaultZipper {
    fn zip(
        &self,
        left_iter: Box<Iterator<Item = Result<String>>>,
        right_iter: Box<Iterator<Item = Result<String>>>,
        output: Box<&mut ZipOutput>,
    ) -> Result<()> {
        for items in left_iter.zip(right_iter) {
            let left = items.0?;
            let right = items.1?;
            output.write(left, right)?;
        }
        output.end()?;
        return Ok(());
    }
}

pub struct CycledZipper {}

impl Zipper for CycledZipper {
    fn zip(
        &self,
        left_iter: Box<Iterator<Item = Result<String>>>,
        right_iter: Box<Iterator<Item = Result<String>>>,
        output: Box<&mut ZipOutput>,
    ) -> Result<()> {
        let left_items: Result<Vec<String>> = left_iter.collect();

        match left_items {
            Ok(l) => {
                let mut count = 0;

                for items in l.iter().cycle().zip(right_iter) {
                    let left = items.0;
                    let right = items.1?;
                    output.write(left.to_owned(), right)?;

                    count += 1;
                    if count == l.len() {
                        count = 0;
                        output.end()?;
                    }
                }

                if count > 0 {
                    eprintln!("WARNING: right input is shorter than left input");
                    output.end()?;
                }
            }
            Err(err) => return Err(err),
        }

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_zip_default() {
        let left_data = vec![Ok("foo".to_owned()), Ok("bar".to_owned())].into_iter();
        let right_data = vec![Ok("a".to_owned()), Ok("b".to_owned())].into_iter();

        let mut output = MockOutput::new();

        let zipper = DefaultZipper {};
        zipper
            .zip(
                Box::new(left_data),
                Box::new(right_data),
                Box::new(&mut output),
            )
            .unwrap();

        assert_eq!(output.end_called, true);

        let mut output_iter = output.buffer.iter();

        assert_eq!(
            output_iter.next(),
            Some(&("foo".to_owned(), "a".to_owned()))
        );
        assert_eq!(
            output_iter.next(),
            Some(&("bar".to_owned(), "b".to_owned()))
        );
        assert_eq!(output_iter.next(), None);
    }

    #[test]
    fn should_zip_cycled() {
        let left_data = vec![Ok("name".to_owned()), Ok("type".to_owned())].into_iter();
        let right_data = vec![
            Ok("fido".to_owned()),
            Ok("dog".to_owned()),
            Ok("mittens".to_owned()),
            Ok("cat".to_owned()),
        ]
        .into_iter();

        let mut output = MockOutput::new();

        let zipper = CycledZipper {};
        zipper
            .zip(
                Box::new(left_data),
                Box::new(right_data),
                Box::new(&mut output),
            )
            .unwrap();

        assert_eq!(output.end_called, true);
        let mut output_iter = output.buffer.iter();

        assert_eq!(
            output_iter.next(),
            Some(&("name".to_owned(), "fido".to_owned()))
        );
        assert_eq!(
            output_iter.next(),
            Some(&("type".to_owned(), "dog".to_owned()))
        );
        assert_eq!(
            output_iter.next(),
            Some(&("name".to_owned(), "mittens".to_owned()))
        );
        assert_eq!(
            output_iter.next(),
            Some(&("type".to_owned(), "cat".to_owned()))
        );

        assert_eq!(output_iter.next(), None);
    }

    #[test]
    fn should_do_nothing_with_empty_inputs() {
        let left_data = Vec::new().into_iter();
        let right_data = Vec::new().into_iter();

        let mut output = MockOutput::new();

        let zipper = DefaultZipper {};
        zipper
            .zip(
                Box::new(left_data),
                Box::new(right_data),
                Box::new(&mut output),
            )
            .unwrap();

        assert_eq!(output.end_called, true);
        assert_eq!(output.buffer.len(), 0);
    }

    #[test]
    fn should_do_nothing_with_empty_inputs_cycled() {
        let left_data = Vec::new().into_iter();
        let right_data = Vec::new().into_iter();

        let mut output = MockOutput::new();

        let zipper = CycledZipper {};
        zipper
            .zip(
                Box::new(left_data),
                Box::new(right_data),
                Box::new(&mut output),
            )
            .unwrap();

        assert_eq!(output.end_called, false);
        assert_eq!(output.buffer.len(), 0);
    }

    struct MockOutput {
        buffer: Vec<(String, String)>,
        end_called: bool,
    }

    impl ZipOutput for MockOutput {
        fn write(&mut self, left: String, right: String) -> Result<()> {
            self.buffer.push((left, right));
            Ok(())
        }
        fn end(&mut self) -> Result<()> {
            self.end_called = true;
            Ok(())
        }
    }

    impl MockOutput {
        fn new() -> MockOutput {
            return MockOutput {
                buffer: Vec::new(),
                end_called: false,
            };
        }
    }

}
