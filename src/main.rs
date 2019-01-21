mod io;
mod zipper;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use structopt::StructOpt;

use io::LineReader;
use zipper::render::{JsonOuput, TabOutput, ZipOutput};
use zipper::{CycledZipper, DefaultZipper, Zipper};

/// Takes a line from each input file and prints them to stdout, until either file reaches EOF
#[derive(StructOpt)]
#[structopt(name = "zipit")]
struct Opt {
    /// Output as JSON
    #[structopt(short = "j", long = "json")]
    json: bool,

    /// Cycle all lines from <LEFT_FILE> indefinitely
    #[structopt(short = "c", long = "cycle")]
    cycle: bool,

    #[structopt(name = "LEFT_FILE")]
    left_file: String,

    #[structopt(name = "RIGHT_FILE")]
    right_file: String,
}

fn open_file_as_line_reader(file: &str) -> Result<LineReader> {
    let buffer = BufReader::new(Box::new(File::open(file)?) as Box<Read>);
    let lr = LineReader::new(buffer);

    return Ok(lr);
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let reader_left = open_file_as_line_reader(opt.left_file.as_str())?;
    let reader_right = open_file_as_line_reader(opt.right_file.as_str())?;
    let mut json_output = JsonOuput::new();
    let mut tabbed_output = TabOutput::new();

    let renderer: Box<&mut ZipOutput> = match opt.json {
        true => Box::new(&mut json_output),
        false => Box::new(&mut tabbed_output),
    };

    let zipper: Box<Zipper> = match opt.cycle {
        true => Box::new(CycledZipper {}),
        false => Box::new(DefaultZipper {}),
    };

    zipper.zip(Box::new(reader_left), Box::new(reader_right), renderer)?;

    return Ok(());
}
