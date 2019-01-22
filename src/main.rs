mod io;
mod zipper;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Result;
use std::process;
use structopt::StructOpt;

use io::LineReader;
use zipper::render::{JsonOuput, TabbedOutput, ZipOutput};
use zipper::{CycledZipper, DefaultZipper, Zipper};

/// Takes a line from two files and prints them to stdout, until either file reaches EOF
#[derive(StructOpt)]
#[structopt(name = "zipit")]
struct Opt {
    /// Output as JSON (warning: requires unique lines in <left_file>)
    #[structopt(short = "j", long = "json")]
    json: bool,

    /// Cycle lines from <left_file> indefinitely
    #[structopt(short = "c", long = "cycle")]
    cycle: bool,

    /// The left data set (required)
    #[structopt(name = "left_file")]
    left_file: String,

    /// The right data set (optional, defaults to stdin (-) if omitted).
    #[structopt(name = "right_file")]
    right_file: Option<String>,
}

fn open_file_as_line_reader(file: &str) -> Result<LineReader> {
    match File::open(file) {
        Ok(f) => {
            let buffer = BufReader::new(Box::new(f) as Box<Read>);
            let lr = LineReader::new(buffer);
            return Ok(lr);
        }
        Err(err) => Err(update_error(format!("failed to open {}", file), err)),
    }
}

fn open_stdin_as_line_reader() -> Result<LineReader> {
    let stdin = Box::leak(Box::new(std::io::stdin()));
    let stdin_lock = stdin.lock();
    let buffer = BufReader::new(Box::new(stdin_lock) as Box<Read>);
    Ok(LineReader::new(buffer))
}

fn update_error(context: String, e: std::io::Error) -> std::io::Error {
    let msg = format!("{} {}", context, e);
    std::io::Error::new(e.kind(), msg)
}

fn run(opt: &Opt) -> Result<()> {
    let reader_left = open_file_as_line_reader(opt.left_file.as_str())?;

    let reader_right: LineReader = match &opt.right_file {
        None => open_stdin_as_line_reader(),
        Some(ref file) if file.as_str() == "-" => open_stdin_as_line_reader(),
        Some(file) => open_file_as_line_reader(file.as_str()),
    }?;

    let mut json_output = JsonOuput::new();
    let mut tabbed_output = TabbedOutput::new();

    let renderer: Box<&mut ZipOutput> = match opt.json {
        true => Box::new(&mut json_output),
        false => Box::new(&mut tabbed_output),
    };

    let zipper: Box<Zipper> = match opt.cycle {
        true => Box::new(CycledZipper {}),
        false => Box::new(DefaultZipper {}),
    };

    zipper.zip(Box::new(reader_left), Box::new(reader_right), renderer)?;

    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    match run(&opt) {
        Ok(()) => process::exit(0),
        Err(err) => {
            eprintln!("ERROR: {}", err);
            process::exit(1);
        }
    }
}
