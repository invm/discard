use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, Read, Write};

/// Read a file and discard everything between the first occurrence of START and the first occurrence of END.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file.
    input: String,

    /// Output file. If not provided, it will write to stdout. 
    #[arg(short)]
    output: Option<String>,

    /// Start pattern to search for.
    #[arg(short)]
    start: String,

    /// End pattern to search for.
    #[arg(short)]
    end: String,

    /// Step size for reading the file.
    #[arg(long, default_value_t = 100000, allow_negative_numbers = false)]
    step: usize,
}

#[derive(Default)]
pub struct Output {
    pub file: Option<File>,
    pub buffer: Vec<u8>,
}

impl Output {
    fn new(output: Option<String>) -> Result<Self, io::Error> {
        let mut file = None;
        if let Some(o) = output {
            file = Some(File::create(o)?);
        }
        Ok(Self {
            file,
            buffer: vec![],
        })
    }

    fn write(&mut self, data: &[u8]) -> io::Result<()> {
        if let Some(ref mut file) = self.file {
            file.write_all(data)?;
        } else {
            self.buffer.extend_from_slice(data);
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut input_file = File::open(&args.input)?;
    let mut output = Output::new(args.output)?;
    let mut buffer = vec![0; args.step];
    let start_re = Regex::new(&args.start).unwrap();
    let end_re = Regex::new(&args.end).unwrap();

    let mut found_start = false;

    loop {
        let n = input_file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        let chunk = String::from_utf8_lossy(&buffer[..n]);

        if let Some(start_match) = start_re.find(&chunk) {
            if found_start {
                // If start is already found, continue searching for end within the chunk
                if let Some(end_match) = end_re.find(&chunk[start_match.end()..]) {
                    let after_end = &chunk[start_match.end() + end_match.end()..];
                    output
                        .write(&[end_match.as_str().as_bytes(), after_end.as_bytes()].concat())?;
                    found_start = false;
                    continue;
                }
            } else {
                // If start is not found yet, write up to the start match
                let before_start = &chunk[..start_match.start()];
                output
                    .write(&[before_start.as_bytes(), start_match.as_str().as_bytes()].concat())?;
                found_start = true;

                if let Some(end_match) = end_re.find(&chunk[start_match.end()..]) {
                    let after_end = &chunk[start_match.end() + end_match.end()..];
                    output
                        .write(&[end_match.as_str().as_bytes(), after_end.as_bytes()].concat())?;
                    found_start = false;
                    continue;
                }
            }
        } else if found_start {
            // If in the middle of discarding, continue searching for end
            if let Some(end_match) = end_re.find(&chunk) {
                let after_end = &chunk[end_match.end()..];
                output.write(&[end_match.as_str().as_bytes(), after_end.as_bytes()].concat())?;
                found_start = false;
            }
        } else {
            // Regular writing if no special conditions
            output.write(chunk.as_bytes())?;
        }
    }
    if output.file.is_none() {
        io::stdout().write_all(&output.buffer)?;
    }

    Ok(())
}
